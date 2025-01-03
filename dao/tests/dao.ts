import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dao } from "../target/types/dao";
import { assert } from "chai";

describe("dao", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;

  anchor.setProvider(provider);

  const program = anchor.workspace.Dao as Program<Dao>;

  const daoStateKeypair = anchor.web3.Keypair.generate();
  const proposalKeypair = anchor.web3.Keypair.generate();
  const memberKeypair = anchor.web3.Keypair.generate();

  it("Initializes the DAO", async () => {
    await program.methods
      .initialize()
      .accounts({
        daoState: daoStateKeypair.publicKey,
        user: wallet.publicKey,
      })
      .signers([daoStateKeypair])
      .rpc();

    const daoState = await program.account.daoState.fetch(
      daoStateKeypair.publicKey
    );
    assert.strictEqual(daoState.treasuryBalance.toNumber(), 0);
    assert.strictEqual(daoState.totalProposals.toNumber(), 0);
  });

  it("Joins the DAO", async () => {
    const membershipFee = new anchor.BN(100);

    await program.methods
      .joinDao(membershipFee)
      .accounts({
        daoState: daoStateKeypair.publicKey,
        memberAccount: memberKeypair.publicKey,
        user: wallet.publicKey,
      })
      .signers([memberKeypair])
      .rpc();

    const daoState = await program.account.daoState.fetch(
      daoStateKeypair.publicKey
    );
    assert.strictEqual(daoState.treasuryBalance.toNumber(), 100);

    const memberAccount = await program.account.memberAccount.fetch(
      memberKeypair.publicKey
    );
    assert.strictEqual(memberAccount.isMember, true);
  });

  it("Creates a proposal", async () => {
    const title = "Increase treasury allocation";
    const description = "Proposal to allocate 500 units to marketing";
    const amount_requested = new anchor.BN(50);

    await program.methods
      .createProposal(title, description, amount_requested)
      .accounts({
        daoState: daoStateKeypair.publicKey,
        proposalAccount: proposalKeypair.publicKey,
        user: wallet.publicKey,
      })
      .signers([proposalKeypair])
      .rpc();

    const daoState = await program.account.daoState.fetch(
      daoStateKeypair.publicKey
    );
    assert.strictEqual(daoState.totalProposals.toNumber(), 1);

    const proposalAccount = await program.account.proposalAccount.fetch(
      proposalKeypair.publicKey
    );
    assert.strictEqual(proposalAccount.title, title);
    assert.strictEqual(proposalAccount.description, description);
    assert.strictEqual(proposalAccount.upvotes.toNumber(), 0);
    assert.strictEqual(proposalAccount.downvotes.toNumber(), 0);
    assert.strictEqual(proposalAccount.amountRequested.toNumber(), 50);
    assert.strictEqual(proposalAccount.isExecuted, false);
  });

  it("Votes on a proposal", async () => {
    await program.methods
      .vote(true) // Vote "for"
      .accounts({
        proposalAccount: proposalKeypair.publicKey,
        user: wallet.publicKey,
      })
      .signers([])
      .rpc();

    const proposalAccount = await program.account.proposalAccount.fetch(
      proposalKeypair.publicKey
    );
    assert.strictEqual(proposalAccount.upvotes.toNumber(), 1);
    assert.strictEqual(proposalAccount.downvotes.toNumber(), 0);
  });

  it("Executes a proposal", async () => {
    const amountRequested = new anchor.BN(50);
    const proposalAccount = await program.account.proposalAccount.fetch(
      proposalKeypair.publicKey
    );

    await program.methods
      .executeProposal()
      .accounts({
        daoState: daoStateKeypair.publicKey,
        proposalAccount: proposalKeypair.publicKey,
      })
      .signers([])
      .rpc();

    const updatedProposal = await program.account.proposalAccount.fetch(
      proposalKeypair.publicKey
    );
    assert.strictEqual(updatedProposal.isExecuted, true);

    const daoState = await program.account.daoState.fetch(
      daoStateKeypair.publicKey
    );
    assert.strictEqual(daoState.treasuryBalance.toNumber(), 50);
  });
});

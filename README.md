# DAO Program

This is a decentralized autonomous organization (DAO) program built on Solana using the Anchor framework. The program allows users to join the DAO, create proposals, vote on them, and execute approved proposals.
To keep things simple, this has no PDAs(Program Derived Address) and no complex logic. This could be used as a reference for building DAOs and to help grasp the concept of DAOs. It uses a simple logic to execute the proposal which is if majority agrees on a proposal then it is executed.
---

## 📂 **Program Structure**

### **Accounts**

#### 1. **DaoState**
Stores the overall state of the DAO.

- **Fields**:
  - `treasury_balance (u64)`: The DAO's treasury balance.
  - `total_proposals (u64)`: Total number of proposals created in the DAO.

#### 2. **MemberAccount**
Stores individual member information.

- **Fields**:
  - `is_member (bool)`: Indicates whether the user is a DAO member.

#### 3. **ProposalAccount**
Represents a proposal created in the DAO.

- **Fields**:
  - `title (String)`: Title of the proposal.
  - `description (String)`: Description of the proposal.
  - `upvotes (u64)`: Number of upvotes the proposal has received.
  - `downvotes (u64)`: Number of downvotes the proposal has received.
  - `amount_requested (u64)`: Amount requested from the DAO treasury.
  - `is_executed (bool)`: Indicates whether the proposal has been executed.

---

## 📜 **Functions**

### 1. **`initialize`**
Initializes the DAO state.

- **Parameters**: None.
- **Effects**:
  - Sets `treasury_balance` to `0`.
  - Sets `total_proposals` to `0`.

---

### 2. **`join_dao`**
Allows a user to join the DAO by paying a membership fee.

- **Parameters**:
  - `membership_fee (u64)`: Fee paid by the user to join the DAO.
- **Effects**:
  - Adds the membership fee to the `treasury_balance`.
  - Marks the user as a member in the `MemberAccount`.

---

### 3. **`create_proposal`**
Creates a new proposal in the DAO.

- **Parameters**:
  - `title (String)`: Title of the proposal.
  - `description (String)`: Description of the proposal.
  - `amount_requested (u64)`: Amount requested from the DAO treasury.
- **Effects**:
  - Increments the `total_proposals` in `DaoState`.
  - Initializes a `ProposalAccount` with the provided details.

---

### 4. **`vote`**
Allows members to vote on a proposal.

- **Parameters**:
  - `approve (bool)`: Indicates whether the member approves or disapproves of the proposal.
- **Effects**:
  - Increments `upvotes` if approved, otherwise increments `downvotes` in `ProposalAccount`.

---

### 5. **`execute_proposal`**
Executes a proposal if it has more upvotes than downvotes and hasn't been executed yet.

- **Parameters**: None.
- **Preconditions**:
  - Proposal must have more `upvotes` than `downvotes`.
  - Proposal must not have already been executed.
- **Effects**:
  - Deducts the `amount_requested` from `treasury_balance`.
  - Marks the proposal as executed in the `ProposalAccount`.

---

## 💡 **How to Use**

1. **Initialize the DAO**: Call `initialize` to set up the DAO state.
2. **Join the DAO**: Users can call `join_dao` and pay a membership fee to join.
3. **Create Proposals**: Members can call `create_proposal` to propose ideas and request funds.
4. **Vote on Proposals**: Members can call `vote` to support or oppose proposals.
5. **Execute Proposals**: If a proposal is approved (more upvotes than downvotes), it can be executed using `execute_proposal`.

---

## 📄 **Error Handling**

### Custom Errors:
1. **`ProposalAlreadyExecuted`**: Proposal has already been executed.
2. **`ProposalRejected`**: Proposal has more downvotes than upvotes and cannot be executed.

---

Feel free to contribute or suggest improvements to this DAO program! 🚀

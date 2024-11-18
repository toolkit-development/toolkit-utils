# **IC Toolkit Utils**

**IC Toolkit Utils** is a Rust crate designed to provide reusable utilities, traits, and models for building scalable and efficient applications on the Internet Computer (IC). It simplifies common development tasks such as access control, cycles and ICP management, wasm validation, logging, and stable storage abstractions.

---

## **Features**

### **1. Helpers**

A collection of utility functions for core IC operations:

- **Candid Save**:

  - Save and load candid data to/from stable storage.

- **Cycles Management**:

  - Convert ICP to cycles and vice versa.
  - Calculate ICP fees for canister spin-ups and operations.

- **Transactions**:

  - Handle ICP transfers, allowances, and balance checks.
  - Notify for top-ups and validate transactions.

- **WASM Management**:

  - Validate and decompress gzipped wasm modules.

- **String Utilities**:

  - Compare strings and measure their lengths.

- **Validation**:
  - Validate inputs like email addresses, string lengths, and date ranges.

---

### **2. Storage Abstractions**

The crate provides abstractions for stable storage to facilitate persistence across upgrades:

- **Stable Cells**:

  - Used for managing single-value stable memory.

- **Stable BTree Maps**:
  - For managing key-value pairs with efficient query and update operations.

---

### **3. Logging**

Structured logging for key system events, including wasm updates, governance events, and transactions:

- Supports detailed logs with metadata on changes (e.g., initial and new values).
- Log entries are serialized for persistence.

---

### **4. Cycles and ICP Management**

- Convert between ICP and cycles efficiently.
- Calculate fees for operations like canister creation or spin-ups.
- Utilities for handling ICP transactions and balance queries.

---

### **5. Data Models**

Reusable structures for managing state and metadata:

- **Logs**: Capture operational events with `Log` and `LogResponse`.
- **Metadata**: Store and manage canister metadata, including ownership and public/private states.
- **Versioning**: Manage semantic versions for upgrades.
- **Validation**: Support for structured validation responses and input types.
- **Paged Responses**: Enable scalable paginated API results.

---

### **6. Miscellaneous Utilities**

- **Hashing**: Compute SHA-256 checksums for wasm modules and other data.
- **Base64 Encoding**: Encode and decode image data for storage or transfer.
- **Generic Constants**: Includes helpers like `ICP_TRANSACTION_FEE` and `TRILLION_CYCLES`.

---

## **How to Add to Your Project**

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
ic-toolkit-utils = { git = "https://github.com/your-repo/ic-toolkit-utils.git" }
```

---

## **Contributing**

Contributions are welcome! Follow these steps to contribute:

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Commit your changes: `git commit -m "Add new feature"`.
4. Push to the branch: `git push origin feature-name`.
5. Submit a pull request.

---

## **License**

This project is licensed under the **GNU GPLv3 License**. See the `LICENSE` file for details.

---

Let me know if further adjustments are needed!

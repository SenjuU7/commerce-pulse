# 🚀 Commerce Pulse

A high-performance ETL (Extract, Transform, Load) pipeline built with Rust and PostgreSQL for retail sales analytics. Built as a learning project to master data engineering concepts with a planned dashboard visualization interface.

## 📊 Features

- **Fast CSV Processing**: Asynchronous data import using Tokio
- **PostgreSQL Integration**: Efficient data storage and querying
- **Type-Safe ETL**: Leveraging Rust's type system for reliable data transformation
- **Progress Tracking**: Real-time feedback during data import
- **Configurable Limits**: Support for partial data loading during development

## 🛠️ Tech Stack

- **Language**: Rust
- **Database**: PostgreSQL
- **Async Runtime**: Tokio
- **CSV Processing**: csv crate
- **Date Handling**: chrono

## 📁 Project Structure

```
commerce-pulse/
├── src/
│   ├── db/
│   │   ├── connect.rs    # Database connection handling
│   │   ├── mod.rs        # Database module exports
│   │   └── query.rs      # SQL queries and table creation
│   ├── etl/
│   │   ├── import.rs     # CSV import and data transformation
│   │   └── mod.rs        # ETL module exports
│   └── main.rs           # Application entry point
├── data/
│   └── sales_trx.csv     # Sample retail transaction data
├── .env.example          # Environment variable template
├── Cargo.toml            # Rust dependencies
└── README.md
```

## 🚀 Getting Started

### Prerequisites

- Rust (1.70+)
- PostgreSQL (12+)
- Git

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/SenjuU7/commerce-pulse.git
   cd commerce-pulse
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` and add your PostgreSQL connection:
   ```
   DATABASE_URL=postgresql://username:password@localhost/dbname
   ```

3. **Create database**
   ```bash
   psql -U postgres
   CREATE DATABASE your_database_name;
   \q
   ```

4. **Build and run**
   ```bash
   cargo build --release
   cargo run
   ```

## 📝 Usage

The application will:
1. Connect to PostgreSQL
2. Create the `sales_transactions` table if it doesn't exist
3. Import CSV data from `data/sales_trx.csv`
4. Display sample results

### Sample Output

```
DB READY 🚀
TABLE READY 🚀
Processed 100 rows
Processed 200 rows
...
Reached limit of 1000 rows

📊 Sales Transactions:
--------------------------------------------------------
581482 | 2019-12-09 08:00 | Set Of 2 Wooden Market Crates | $21.47 x 12 | United Kingdom
...
--------------------------------------------------------
✅ Total rows: 1000
```

## 🗄️ Database Schema

```sql
CREATE TABLE sales_transactions (
    TransactionNo VARCHAR(50) PRIMARY KEY,
    Date TIMESTAMP NOT NULL,
    ProductNo VARCHAR(50) NOT NULL,
    ProductName VARCHAR(255) NOT NULL,
    Price DOUBLE PRECISION NOT NULL,
    Quantity INTEGER NOT NULL,
    CustomerNo VARCHAR(50) NOT NULL,
    Country VARCHAR(100) NOT NULL
);
```

## 🎯 Roadmap

- [x] CSV import pipeline
- [x] PostgreSQL integration
- [x] Basic data queries
- [ ] Web dashboard with data visualization
- [ ] Advanced analytics (sales trends, top products, geographic analysis)
- [ ] Real-time data streaming
- [ ] API endpoints for external access
- [ ] Docker containerization

## 🤝 Contributing

This is a learning project, but suggestions and feedback are welcome! Feel free to open an issue or submit a pull request.

## 📄 License

MIT License - feel free to use this project for learning purposes.

## 👤 Author

**SenjuU7**
- GitHub: [@SenjuU7](https://github.com/SenjuU7)

## 🙏 Acknowledgments

- Built while learning ETL concepts and Rust
- Inspired by real-world retail analytics needs
- Special thanks to the Rust and PostgreSQL communities

---

⭐ Star this repo if you find it helpful for your learning journey!
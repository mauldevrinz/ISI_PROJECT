Mantap — berikut aku buatin draft `README.md`-nya biar project kamu rapi dan bisa dipakai orang lain (atau buat dokumentasi tugas). Nanti tinggal disesuaikan kalau ada tambahan.

---

## 📖 README.md — Modbus SHT20 Reader via USB to RS485 (Rust)

### 📋 Deskripsi  
Project ini bertujuan untuk membaca data dari sensor **SHT20 versi Modbus RTU** menggunakan USB to RS485 converter (CH340/CH341) ke komputer berbasis **Linux (Ubuntu)** dengan bahasa pemrograman **Rust**. Data suhu dan kelembaban akan ditampilkan di terminal setiap 2 detik sekali.

---

## 🛠️ Instalasi & Setup

### 1️⃣ Install Dependensi Rust

Pastikan **Rust** sudah terinstal. Jika belum:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Cek versi:
```bash
rustc --version
```

### 2️⃣ Install Library Tambahan

Di dalam project, buat project baru atau gunakan yang sudah ada:
```bash
cargo new modbus_sht20
cd modbus_sht20
```

Edit `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-serial = "5.4.4"
tokio-modbus = "0.7"
```

Lalu jalankan:
```bash
cargo build
```

---

## 🔌 Wiring Diagram

| USB to RS485 | SHT20 Modbus |
|:-------------|:--------------|
| A            | A            |
| B            | B            |
| GND          | GND          |
| +5V (opsional) | +5V (jika sensor tidak punya power supply eksternal)|

**Catatan**: Biasanya bisa ambil power langsung dari pin `+5V` USB converter.

---

## ⚙️ Konfigurasi Linux

1. Cek koneksi device:
   ```bash
   sudo dmesg | grep tty
   ```
   Pastikan terbaca misal: `/dev/ttyUSB0`

2. Disable BRLTTY jika bentrok:
   ```bash
   sudo systemctl stop brltty
   sudo systemctl disable brltty
   sudo apt remove --purge brltty
   ```

---

## 🚀 Menjalankan Program

1. Simpan kode program ke file `src/main.rs`
2. Jalankan program:
   ```bash
   cargo run
   ```

---

## 📊 Output

Terminal akan menampilkan data seperti:
```
Temperature: 29.3°C
Humidity: 75.6%
```
**Update setiap 2 detik.**

---

## 📌 Catatan Tambahan

- Pastikan baudrate, alamat slave, dan mapping register sesuai dengan konfigurasi sensor.
- Jika muncul error seperti:
  ```
  ExceptionResponse { function: 3, exception: IllegalDataAddress }
  ```
  Artinya alamat register yang diminta belum sesuai dengan tabel register sensor.

---

## 📚 Referensi

- [Tokio Serial](https://docs.rs/tokio-serial)
- [Tokio Modbus](https://docs.rs/tokio-modbus)
- Datasheet SHT20 Modbus RTU

---

Kalau kamu mau aku bantu sekalian bikinin template project-nya sekalian (beserta file `Cargo.toml` dan struktur folder-nya), tinggal bilang aja ya bro! 🚀

# 📚 Katalog Perpustakaan Rust (Simulasi Ownership)

Ini adalah proyek simulasi katalog perpustakaan sederhana yang dibuat dengan Rust. Proyek ini berfungsi sebagai latihan mendalam untuk menjelajahi konsep kepemilikan (ownership) dan pola desain *smart pointer* di Rust.

Tujuan utamanya bukan hanya untuk membuat katalog, tetapi untuk memodelkan skenario dunia nyata yang kompleks:
**"Bagaimana jika satu data (sebuah `Buku`) perlu dimiliki dan diubah oleh beberapa 'pemilik' (Katalog *dan* Anggota) secara bersamaan?"**

---

## 💡 Konsep Rust yang Diterapkan

Proyek ini secara khusus dirancang untuk mempraktikkan konsep-konsep inti dan lanjutan Rust:

* **`HashMap<K, V>`**: Digunakan sebagai struktur data inti untuk `katalog`, memetakan ISBN (`String`) ke data buku.
* **`struct`** dan **`enum`**: Digunakan untuk pemodelan data yang bersih (`Buku`, `Anggota`, `StatusBuku`).
* **`Ownership`** & **`Borrowing`**: Fondasi dari seluruh logika, terutama `&mut Anggota` dan `&HashMap`.
* **`Rc<T>` (Reference Counting)**:  Ini adalah *smart pointer* yang memungkinkan kepemilikan bersama. Ini adalah "kunci" yang memungkinkan `Katalog` dan `Anggota` sama-sama "memiliki" referensi ke `Buku` yang sama.
* **`RefCell<T>` (Interior Mutability)**:  Karena `Rc<T>` hanya memberikan referensi *immutable*, kita membungkus `Buku` dengan `RefCell` untuk mendapatkan "mutabilitas internal". Ini memungkinkan kita untuk mengubah `status` buku (misal: dari `Tersedia` menjadi `Dipinjam`) meskipun buku itu sedang dimiliki bersama.
* **Pola `Rc<RefCell<T>>`**: Menggabungkan kedua *smart pointer* di atas untuk menciptakan data yang dapat dimiliki bersama *dan* dapat diubah—sebuah pola yang sangat umum dan kuat di Rust.
* **`match` & `borrow()`/`borrow_mut()`**: Menggunakan *pattern matching* dan metode dari `RefCell` untuk mengelola status peminjaman dengan aman (pengecekan peminjaman terjadi saat *run-time*).

---

## 🚀 Cara Menjalankan

Anda hanya perlu Rust dan Cargo terinstal.

1.  **Clone repositori:**
    ```bash
    git clone https://github.com/achmadichzan/pustaka.git
    ```

2.  **Masuk ke direktori:**
    ```bash
    cd [NAMA_FOLDER_PROYEK_ANDA]
    ```

3.  **Jalankan program:**
    ```bash
    cargo run
    ```

---

## 🖥️ Contoh Output

Menjalankan `cargo run` akan menyimulasikan seluruh skenario:

```bash
Katalog Awal:
{
    "978-3": RefCell { ... "40 Hukum Kekuasaan", status: Tersedia ... },
    "978-2": RefCell { ... "Belajar Kotlin", status: Tersedia ... },
    "978-1": RefCell { ... "Belajar Rust", status: Tersedia ... },
}

--- Sesi Pinjam ---
Sukses meminjam 'Belajar Rust'
Sukses meminjam 'Belajar Kotlin'
Sukses meminjam '40 Hukum Kekuasaan'
Maaf, 'Belajar Rust' sudah dipinjam oleh Hilmy
Maaf, buku dengan ISBN '000-0' tidak ditemukan.

Katalog Akhir:
{
    "978-3": RefCell { ... "40 Hukum Kekuasaan", status: Dipinjam("Hilmy") ... },
    "978-2": RefCell { ... "Belajar Kotlin", status: Dipinjam("Ikram") ... },
    "978-1": RefCell { ... "Belajar Rust", status: Dipinjam("Hilmy") ... },
}

Anggota Hilmy:
Anggota {
    nama: "Hilmy",
    pinjaman: [
        RefCell { ... "Belajar Rust", status: Dipinjam("Hilmy") ... },
        RefCell { ... "40 Hukum Kekuasaan", status: Dipinjam("Hilmy") ... },
    ],
}

Anggota Ikram:
Anggota {
    nama: "Ikram",
    pinjaman: [
        RefCell { ... "Belajar Kotlin", status: Dipinjam("Ikram") ... },
    ],
}
```
## 🔮 Pengembangan Selanjutnya

Proyek ini dapat dengan mudah diperluas dengan fitur-fitur berikut:

* **Fungsi** `kembalikan_buku` Membuat fungsi yang menghapus buku dari `Vec` pinjaman `Anggota` dan mengubah `status` buku kembali menjadi `Tersedia`.
* **Penanganan Error dengan** `Result`: Mengubah `fn pinjam_buku` agar mengembalikan `Result<(), String>` (sukses atau pesan error) alih-alih mencetak ke konsol.
* **Penyimpanan Persisten:** Menggunakan *crate* `serde_json` untuk menyimpan dan membaca status `katalog` dari sebuah file katalog.json.
* **Input Pengguna (CLI):** Menggunakan *crate* `clap` untuk membuat antarmuka baris perintah (CLI) yang interaktif.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Buku {
    judul: String,
    status: StatusBuku
}

#[derive(Debug)]
struct Anggota {
    nama: String,
    pinjaman: Vec<Rc<RefCell<Buku>>>
}

#[derive(Debug)]
enum StatusBuku {
    Tersedia,
    Dipinjam(String)
}

fn main() {
    let mut katalog: HashMap<String, Rc<RefCell<Buku>>> = HashMap::new();

    let belajar_rust = Buku {
        judul: "Belajar Rust".to_string(),
        status: StatusBuku::Tersedia,
    };

    let belajar_kotlin = Buku {
        judul: "Belajar Kotlin".to_string(),
        status: StatusBuku::Tersedia,
    };

    let fourty_laws_of_power = Buku {
        judul: "40 Hukum Kekuasaan".to_string(),
        status: StatusBuku::Tersedia,
    };

    katalog.insert("978-1".to_string(), Rc::new(RefCell::new(belajar_rust)));
    katalog.insert("978-2".to_string(), Rc::new(RefCell::new(belajar_kotlin)));
    katalog.insert("978-3".to_string(), Rc::new(RefCell::new(fourty_laws_of_power)));

    println!("Katalog Awal:\n{:#?}", katalog);

    let mut hilmy = Anggota {
        nama: "Hilmy".to_string(),
        pinjaman: Vec::new(),
    };

    let mut ikram = Anggota {
        nama: "Ikram".to_string(),
        pinjaman: Vec::new(),
    };

    println!("\n--- Sesi Pinjam ---");
    pinjam_buku(&katalog, &mut hilmy, "978-1");
    pinjam_buku(&katalog, &mut ikram, "978-2");
    pinjam_buku(&katalog, &mut hilmy, "978-3");

    pinjam_buku(&katalog, &mut ikram, "978-1");
    pinjam_buku(&katalog, &mut hilmy, "000-0");

    println!("\nKatalog Akhir:\n{:#?}", katalog);
    println!("\nStatus Anggota Hilmy:\n{:#?}", hilmy);
    println!("\nStatus Anggota Ikram:\n{:#?}", ikram);
}

fn pinjam_buku(
    katalog: &HashMap<String, Rc<RefCell<Buku>>>,
    anggota: &mut Anggota,
    isbn: &str
) {
    match katalog.get(isbn) {
        None => {
            println!("Maaf, buku dengan ISBN '{}' tidak ditemukan.", isbn);
        }
        Some(buku_rc) => {
            let buku = buku_rc.borrow();

            match &buku.status {
                StatusBuku::Tersedia => {
                    drop(buku);

                    let mut buku_mut = buku_rc.borrow_mut();

                    println!("Sukses meminjam '{}'", buku_mut.judul);
                    buku_mut.status = StatusBuku::Dipinjam(anggota.nama.clone());

                    anggota.pinjaman.push(Rc::clone(buku_rc))
                }
                StatusBuku::Dipinjam(peminjam) => {
                    println!("Maaf, '{}' sudah dipinjam oleh '{}'", buku.judul, peminjam);
                }
            }
        }
    }
}
enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

fn print_publications(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(ref book) => {
                println!(
                    "Kitap: {} yazar: {}, {} sayfa",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(ref magazine) => {
                println!(
                    "Dergi: {} - Sayı: {}, Konu: {}",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}

fn main() {
    let book_example = Book {
        title: "Suç ve Ceza".to_string(),
        author: "Fyodor Mihayloviç Dostoyevski".to_string(),
        page_count: 600,
    };

    let magazine_example = Magazine {
        title: "InterWebs".to_string(),
        issue: 42,
        topic: "New player in the Web3 world: kodzar".to_string(),
    };

    let publications = vec![
        Publication::Book(book_example), 
        Publication::Magazine(magazine_example)
    ];

    print_publications(publications);
}
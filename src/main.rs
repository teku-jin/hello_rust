enum Statut {
    Connecte,
    Deconnecte,
    Absent(String),
}

fn main() {
    let etat = Statut::Absent(String::from("En vacances"));
}






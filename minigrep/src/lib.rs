use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenu = fs::read_to_string(config.file)?;

    let resultat = if config.sensible_casse {
        rechercher(&config.recherche, &contenu)
    } else {
        rechercher_insensible_casse(&config.recherche, &contenu)
    };

    for ligne in resultat {
        println!("{}", ligne)
    }

    Ok(())
}

pub struct Config {
    pub recherche: String,
    pub file: String,
    pub sensible_casse: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Il n'y a pas assez d'argument");
        }
        let recherche = args[1].clone();
        let file = args[2].clone();
        let sensible_casse = env::var("MINIGREP_INSENSIBLE_CASSE").is_err();

        Ok(Config {
            recherche,
            file,
            sensible_casse,
        })
    }
}

pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    let mut resultats = Vec::new();
    for ligne in contenu.lines() {
        if ligne.contains(recherche) {
            resultats.push(ligne);
        }
    }
    resultats
}

pub fn rechercher_insensible_casse<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    let recherche = recherche.to_lowercase();
    let mut resultats = Vec::new();
    for ligne in contenu.lines() {
        if ligne.to_lowercase().contains(&recherche) {
            resultats.push(ligne);
        }
    }
    resultats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensible_casse() {
        let recherche = "duct";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.";

        assert_eq!(
            vec!["sécurité, rapidité, productivité."],
            rechercher(recherche, contenu)
        );
    }

    #[test]
    fn insensible_casse() {
        let recherche = "rUsT";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";

        assert_eq!(
            vec!["Rust:", "C'est pas rustique."],
            rechercher_insensible_casse(recherche, contenu)
        );
    }
}

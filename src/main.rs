use num_bigint::BigUint;
use num_traits::One;
use std::io::{self, Write};
use std::time::Instant;

fn calculer_public(g: &BigUint, privee: &BigUint, p: &BigUint) -> BigUint {
    g.modpow(privee, p)
}

fn calculer_secret(public_autre: &BigUint, privee: &BigUint, p: &BigUint) -> BigUint {
    public_autre.modpow(privee, p)
}

fn attaque_force_brute(g: &BigUint, h: &BigUint, p: &BigUint) -> Option<BigUint> {
    let mut a = BigUint::one();
    let p_minus_2 = p - BigUint::one();
    
    while a <= p_minus_2 {
        if g.modpow(&a, p) == *h {
            return Some(a);
        }
        a += BigUint::one();
    }
    None
}

fn lire_nombre(prompt: &str) -> u64 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    println!("\n");
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║     DIFFIE-HELLMAN - DÉMO INTERACTIVE - PROJET RP2026          ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!("\n");
    
    // ============================================================
    // ÉTAPE 1 : Choix des paramètres publics
    // ============================================================
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│  PARAMÈTRES PUBLICS (Tout le monde les connaît)             │");
    println!("└─────────────────────────────────────────────────────────────┘");
    println!();
    println!(" Luis, Charles ET Ismaël connaissent p et g.");
    println!();
    
    let p: u64 = lire_nombre("    Choisis un nombre PREMIER p (ex: 97, 13, 23, 101) : ");
    let g: u64 = lire_nombre("    Choisis un générateur g (ex: 2, 3, 5) : ");
    
    let p_big = BigUint::from(p);
    let g_big = BigUint::from(g);
    
    println!();
    println!("    Paramètres enregistrés : p = {}, g = {}", p, g);
    println!("   (Ismaël les connaît aussi)");
    println!();
    
    // ============================================================
    // Luis choisit sa clé privée
    // ============================================================
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ LUIS choisit sa clé PRIVÉE (secrète)                        │");
    println!("└─────────────────────────────────────────────────────────────┘");
    println!();
    println!("  Seule Luis connaît cette valeur. Ismaël ne la voit PAS ");
    println!();
    
    let a = lire_nombre("   Clé privée de Luis a (entre 1 et p-2) : ");
    let a_big = BigUint::from(a);
    
    println!();
    println!("    Luis garde a = {} secrète.", a);
    println!();
    
    // ============================================================
    // Charles choisit sa clé privée
    // ============================================================
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ CHARLES choisit sa clé PRIVÉE (secrète)                     │");
    println!("└─────────────────────────────────────────────────────────────┘");
    println!();
    
    let b = lire_nombre("    Clé privée de Bob b (entre 1 et p-2) : ");
    let b_big = BigUint::from(b);
    
    println!();
    println!("   Bob garde b = {} secrète.", b);
    println!();
    
    // ============================================================
    // ÉTAPE 4 : Calcul et échange des clés PUBLIQUES
    // ============================================================
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│  Échange des clés PUBLIQUES (Ismaël voit ça)                │");
    println!("└─────────────────────────────────────────────────────────────┘");
    println!();
    
    let A = calculer_public(&g_big, &a_big, &p_big);
    let B = calculer_public(&g_big, &b_big, &p_big);
    
    println!("    Luis calcule : A = g^a mod p = {}^({}) mod {} = {}", g, a, p, A);
    println!("    Luis envoie A = {} à Charles (Ismaël intercepte )", A);
    println!();
    println!("    Charles calcule   : B = g^b mod p = {}^({}) mod {} = {}", g, b, p, B);
    println!("    Charles envoie B = {} à Luis (Ismaël intercepte )", B);
    println!();
    
    // ============================================================
    // ÉTAPE 5 : Calcul du secret partagé
    // ============================================================
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│  Luis et Charles calculent le secret partagé                │");
    println!("└─────────────────────────────────────────────────────────────┘");
    println!();
    
    let secret_luis = calculer_secret(&B, &a_big, &p_big);
    let secret_charles = calculer_secret(&A, &b_big, &p_big);
    
    println!("   Luis calcule : secret = B^a mod p = {}^({}) mod {} = {}", B, a, p, secret_luis);
    println!("   Charles calcule   : secret = A^b mod p = {}^({}) mod {} = {}", A, b, p, secret_charles);
    println!();
    
    if secret_luis == secret_charles {
        println!("    Secret partagé = {} (Luis et Charles ont la même valeur)", secret_luis);
    } else {
        println!("    ERREUR ! Les secrets ne correspondent pas !");
    }
    println!();
    
    // ============================================================
    //  Ismaël attaque 
    // ============================================================
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│  ISMAËL attaque  (Il a tout écouté)                         │");
    println!("└─────────────────────────────────────────────────────────────┘");
    println!();
    
    println!("   Ismaël connaît : p = {}, g = {}, A = {}", p, g, A);
    println!("   Il cherche a tel que {}^a mod {} = {}", g, p, A);
    println!();
    
    println!("    LANCEMENT DE L'ATTAQUE PAR FORCE BRUTE...");
    let start = Instant::now();
    match attaque_force_brute(&g_big, &A, &p_big) {
        Some(trouve) => {
            let duree = start.elapsed();
            println!();
            println!("   ╔════════════════════════════════════════════════════════════╗");
            println!("   ║    ISMAËL A RÉUSSI L'ATTAQUE                               ║");
            println!("   ╚════════════════════════════════════════════════════════════╝");
            println!();
            println!("    Clé privée de Luis trouvée : a = {}", trouve);
            println!("    Temps de l'attaque : {:?}", duree);
            println!();
            println!("   Ismaël peut maintenant calculer le secret partagé :");
            let secret_ismael = calculer_secret(&B, &trouve, &p_big);
            println!("      secret = B^a mod p = {}^({}) mod {} = {}", B, trouve, p, secret_ismael);
        }
        None => {
            let duree = start.elapsed();
            println!();
            println!("   L'attaque n'a rien trouvé après {:?}... (p est trop grand)", duree);
            println!("   C'est la FRONTIÈRE : avec p grand, l'attaque est impossible !");
        }
    }
    println!();
    
    // ============================================================
    // Déterminer si l'attaque a réussi
    // ============================================================
    let attaque_reussie = if p < 1000 { "RÉUSSI" } else { "ÉCHOUÉ (p trop grand)" };

    println!("══════════════════════════════════════════════════════════════════");
    println!("                      FIN DE LA DÉMONSTRATION                      ");
    println!("══════════════════════════════════════════════════════════════════");
    println!();
}
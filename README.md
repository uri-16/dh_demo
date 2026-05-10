# DH-Demo - Projet RP2026

## Démonstration interactive de Diffie-Hellman

Ce projet répond à la question de **Bobo** (élève de Terminale C) :

> *« Quelle est la frontière objective de non efficacité face à Ismaël du protocole de Diffie-Hellman quant au choix des nombres premiers ? »*

## Objectif

Montrer de manière **interactive** et **visuelle** pourquoi le choix de la taille du nombre premier `p` est crucial pour la sécurité de Diffie-Hellman.

##  Personnages

 
| **Luis** | Choisit une clé privée secrète `a` |
| **Charles** | Choisit une clé privée secrète `b` |
| **Ismaël** | Écoute tout et essaie de casser le secret |

**Vous** = tu joues tous les rôles !

##  Installation

### Prérequis
- [Rust](https://rustup.rs/) installé

### Cloner et exécuter

```bash
git clone https://github.com/uri-16/dh_demo.git
cd dh_demo
cargo run

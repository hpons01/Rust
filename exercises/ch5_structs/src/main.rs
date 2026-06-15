// Mini-projet : SensorLog — mini-datalogger embarqué (style BME280)
//
// Contexte : les données brutes arrivent en entiers depuis le bus I2C.
// Tu vas modéliser une lecture capteur, implémenter ses méthodes,
// puis utiliser la struct update syntax pour produire une variante.
//
// Objectifs :
//   1. Déclarer un struct avec des champs typés
//   2. Dériver Debug
//   3. Écrire un constructeur associé (::new)
//   4. Écrire des méthodes &self et &mut self
//   5. Utiliser la struct update syntax
//
// Commandes :
//   cargo test          → lance tous les tests
//   cargo test -- --nocapture  → affiche les println! aussi

fn main() {
    // Zone libre — expérimente ici, les tests sont en bas du fichier.
    // Exemple une fois l'étape 1 et 2 faites :
    // let r = SensorReading::new(2534, 6421, 101325);
    // println!("{r:?}");
    // println!("Temp: {:.2}°C", r.temperature_celsius());
}

// ============================================================
// ÉTAPE 1 — Définir le struct
// ============================================================
//
// Déclare un struct `SensorReading` avec ces champs :
//
//   temperature_raw : i32   — en centièmes de degrés (ex: 2534 = 25.34 °C)
//   humidity_raw    : u32   — en centièmes de %      (ex: 6421 = 64.21 %)
//   pressure_raw    : u32   — en Pascals             (ex: 101325 Pa)
//   valid           : bool  — false si la lecture est hors-plage ou corrompue
//
// Ajoute aussi #[derive(Debug)] au-dessus du struct.

// TODO: déclare SensorReading ici
#[derive(Debug)]
struct SensorReading {
    temperature_raw : i32,
    humidity_raw : u32,
    pressure_raw : u32,
    valid : bool,
}
// ============================================================
// ÉTAPE 2 — Implémenter le bloc impl
// ============================================================
//
// Écris un bloc `impl SensorReading` avec ces cinq éléments :
//
// a) Constructeur associé (pas de `self` !) :
//      fn new(temperature_raw: i32, humidity_raw: u32, pressure_raw: u32) -> Self
//    La valeur `valid` vaut true seulement si toutes ces conditions sont réunies :
//      - temperature_raw entre -4000 et 8500 (inclus)
//      - humidity_raw <= 10000
//      - pressure_raw > 0
//
// b) Lecture de la température :
//      fn temperature_celsius(&self) -> f32
//    temperature_raw est en centièmes de degrés → divise par 100.0
//
// c) Lecture de l'humidité :
//      fn humidity_percent(&self) -> f32
//    humidity_raw est en centièmes de % → divise par 100.0
//
// d) Accesseur booléen :
//      fn is_valid(&self) -> bool
//    Retourne simplement le champ `valid`.
//
// e) Méthode de mutation :
//      fn invalidate(&mut self)
//    Met `valid` à false (simule une trame I2C corrompue).

// TODO: implémente impl SensorReading ici
impl SensorReading {
    fn new(temperature_raw : i32, humidity_raw: u32, pressure_raw: u32) -> Self {
        Self {
            temperature_raw,
            humidity_raw,
            pressure_raw,
            valid : {
                (temperature_raw <= 8_500) &&
                (temperature_raw >= -4_000) &&
                (humidity_raw <= 10_000) &&
                (pressure_raw > 0)
            },
        }
    }

    fn temperature_celsius(&self) -> f32 {
        self.temperature_raw as f32 / 100.0
    }

    fn humidity_percent(&self) -> f32 {
        self.humidity_raw as f32 / 100.0
    }

    fn is_valid(&self) -> bool {
        self.valid
    }

    fn invalidate(&mut self) {
        self.valid = false;
    }
}
// ============================================================
// ÉTAPE 3 — Struct update syntax
// ============================================================
//
// Complète cette fonction : elle retourne une nouvelle lecture
// identique à `base` mais avec une pression différente.
//
// Utilise la struct update syntax :
//   SensorReading { pressure_raw: new_pressure, ..base }
//
// Pourquoi ça marche sans clone() ici ? Réfléchis aux types des champs.

fn clone_with_new_pressure(base: SensorReading, new_pressure: u32) -> SensorReading {
    // TODO: remplace todo!() par la struct update syntax
    SensorReading {
        pressure_raw : new_pressure,
        ..base
    }
}

// ============================================================
// Tests — ne modifie pas cette section
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let r = SensorReading::new(2534, 6421, 101325);
        assert!(r.is_valid(), "Lecture dans les plages → valid doit être true");
    }

    #[test]
    fn test_new_invalid_temp_too_high() {
        let r = SensorReading::new(9000, 5000, 101325);
        assert!(!r.is_valid(), "Température hors plage → valid doit être false");
    }

    #[test]
    fn test_new_invalid_temp_too_low() {
        let r = SensorReading::new(-5000, 5000, 101325);
        assert!(!r.is_valid(), "Température trop basse → valid doit être false");
    }

    #[test]
    fn test_new_invalid_humidity_too_high() {
        let r = SensorReading::new(2000, 10001, 101325);
        assert!(!r.is_valid(), "Humidité > 100 % → valid doit être false");
    }

    #[test]
    fn test_new_invalid_pressure_zero() {
        let r = SensorReading::new(2000, 5000, 0);
        assert!(!r.is_valid(), "Pression = 0 Pa → invalid");
    }

    #[test]
    fn test_temperature_celsius() {
        let r = SensorReading::new(2534, 6421, 101325);
        let t = r.temperature_celsius();
        assert!((t - 25.34).abs() < 0.001, "Attendu 25.34 °C, obtenu {t}");
    }

    #[test]
    fn test_humidity_percent() {
        let r = SensorReading::new(2534, 6421, 101325);
        let h = r.humidity_percent();
        assert!((h - 64.21).abs() < 0.001, "Attendu 64.21 %, obtenu {h}");
    }

    #[test]
    fn test_invalidate() {
        let mut r = SensorReading::new(2534, 6421, 101325);
        assert!(r.is_valid(), "Avant invalidate : doit être valid");
        r.invalidate();
        assert!(!r.is_valid(), "Après invalidate() : valid doit être false");
    }

    #[test]
    fn test_struct_update_syntax() {
        let base = SensorReading::new(2534, 6421, 101325);
        let updated = clone_with_new_pressure(base, 98000);
        assert_eq!(updated.pressure_raw, 98000, "La pression doit être mise à jour");
        assert_eq!(updated.temperature_raw, 2534, "La température ne change pas");
        assert_eq!(updated.humidity_raw, 6421, "L'humidité ne change pas");
    }

    #[test]
    fn test_debug_derive() {
        let r = SensorReading::new(2000, 5000, 101325);
        let s = format!("{r:?}");
        assert!(
            s.contains("SensorReading"),
            "#[derive(Debug)] doit afficher le nom du struct"
        );
    }
}

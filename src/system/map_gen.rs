use std::fs;

use bevy::ecs::component::Component;

#[derive(Debug, Clone, Component)]
pub struct MapBlock {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub h: f32,
    pub d: f32,
}

impl MapBlock {
    /// Renvoie true si le bloc est valide (dimensions non nulles)
    pub fn is_valid(&self) -> bool {
        self.w != 0.0 && self.h != 0.0 && self.d != 0.0
    }
}

pub fn gen_map(cell_width: f32, block_height: f32) -> Vec<Vec<MapBlock>> {
    // Lecture du fichier de map
    let content = fs::read_to_string("map.txt").expect("File not found");
    // Définir la taille d'une cellule
    // Créer une matrice booléenne : true si le caractère est 'x', false sinon.
    let mask = content
        .lines() // mieux que split("\n") pour éviter les lignes vides en fin de fichier
        .map(|line| {
            line.chars()
                .map(|c| c == 'x')
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let rows = mask.len();
    let cols = if rows > 0 { mask[0].len() } else { 0 };

    // Pour centrer l'origine (0,0) au centre de la map,
    // on calcule un offset qui correspond à la moitié de la largeur/hauteur totale.
    let offset_x = (cols as f32 * cell_width) / 2.0;
    let offset_z = (rows as f32 * cell_width) / 2.0;

    // Construire la matrice de MapBlock
    let mut matrix: Vec<Vec<MapBlock>> = Vec::with_capacity(rows);
    for (row_index, row) in mask.iter().enumerate() {
        let mut row_vec: Vec<MapBlock> = Vec::with_capacity(cols);
        for (col_index, &is_wall) in row.iter().enumerate() {
            // Calculer la position centrale de la cellule.
            // La coordonnée x est calculée en fonction de la colonne,
            // et z en fonction de la ligne.
            let x = col_index as f32 * cell_width + (cell_width) / 2.0 - offset_x;
            let z = row_index as f32 * cell_width + (cell_width) / 2.0 - offset_z;
            // Pour cet exemple, on fixe y à 2.5 (comme dans vos tuples)
            let y = 2.2;
            // Pour un mur, on attribue la taille complète de la cellule,
            // sinon on laisse toutes les dimensions à 0.
            let (w, h, d) = if is_wall {
                (cell_width, block_height, cell_width)
            } else {
                (0.0, 0.0, 0.0)
            };

            row_vec.push(MapBlock { x, y, z, w, h, d });
        }
        matrix.push(row_vec);
    }

    matrix
}

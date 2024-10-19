use std::{fs, marker::PhantomData};

use macroquad::prelude::*;

use crate::assets::asset_path;

pub struct Shader<'a> {
    pub vert: String,
    pub frag: String,
    phantom: PhantomData<&'a String>,
}

impl<'a> Shader<'a> {
    pub fn new(vert: &str, frag: &str) -> Self {
        let mut asset_path = asset_path();
        asset_path.push("shaders");
        let mut vert_path = asset_path.clone();
        vert_path.push(vert);
        let mut frag_path = asset_path.clone();
        frag_path.push(frag);

        Shader {
            vert: fs::read_to_string(vert_path).unwrap(),
            frag: fs::read_to_string(frag_path).unwrap(),
            phantom: PhantomData,
        }
    }

    pub fn to_source(self: &'a Self) -> ShaderSource<'a> {
        let source: ShaderSource<'a> = ShaderSource::Glsl {
            vertex: self.vert.as_str(),
            fragment: self.frag.as_str(),
        };
        source
    }
}

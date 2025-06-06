#[derive(Debug)]
pub enum Key {
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  ArrowDown,
  Space
}

pub fn convert_key(key: String) -> Option<Key> {
  match key.as_str() {
    "ArrowLeft"  => { Some(Key::ArrowLeft)  }
    "ArrowRight" => { Some(Key::ArrowRight) }
    "ArrowUp"    => { Some(Key::ArrowUp)    }
    "ArrowDown"  => { Some(Key::ArrowDown)  }
    " "          => { Some(Key::Space)      }
    _ => None
  }
}
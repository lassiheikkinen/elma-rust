extern crate elma;
extern crate rand;
#[cfg(test)]
mod tests {
    use elma::{ lev, rec, Position };
    use rand::random;

    #[test]
    fn test_decrypt_encrypt () {
        let mut initial: Vec<u8> = vec![];
        for _ in 0..688 {
            initial.push(random::<u8>());
        }
        let decrypted = lev::crypt_top10(&initial);
        let encrypted = lev::crypt_top10(&decrypted);
        assert_eq!(initial, encrypted);
    }

    #[test]
    // Probably redundant, but maybe some new fields are added in the future.
    // Doesn't hurt or impact anything.
    fn level_default_values () {
        let level = lev::Level::new();
        assert_eq!(level.version, lev::Version::Elma);
        assert_eq!(level.link, 0);
        assert_eq!(level.integrity, [0_f64; 4]);
        assert_eq!(level.name, "");
        assert_eq!(level.lgr, "default");
        assert_eq!(level.ground, "ground");
        assert_eq!(level.sky, "sky");
        assert_eq!(level.polygons, vec![]);
        assert_eq!(level.objects, vec![]);
        assert_eq!(level.pictures, vec![]);
        assert_eq!(level.top10_single, vec![]);
        assert_eq!(level.top10_multi, vec![]);
    }

    #[test]
    fn load_valid_level_1 () {
        let level = lev::Level::load_level("tests/test_1.lev");
        assert_eq!(level.version, lev::Version::Elma);
        assert_eq!(level.link, 1524269776);
        assert_eq!(level.integrity, [-1148375.210607791,
                                      1164056.210607791,
                                      1162467.210607791,
                                      1162283.210607791]);
        assert_eq!(level.name, "Rust test");
        assert_eq!(level.lgr, "default");
        assert_eq!(level.ground, "ground");
        assert_eq!(level.sky, "sky");

        // Polygon tests.
        assert_eq!(level.polygons.len(), 2);
        assert_eq!(level.polygons, vec![lev::Polygon {
                grass: false, vertices: vec![
                    Position { x: -23.993693053024586, y: -3.135779367971911 },
                    Position { x: -15.989070625361132, y: -3.135779367971911 },
                    Position { x: -15.989070625361132, y: 1.995755366905195 },
                    Position { x: -24f64, y: 2f64 }]
            },
            lev::Polygon {
                grass: true, vertices: vec![
                    Position { x: -23.83645939819548, y: 2.310222676563402 },
                    Position { x: -17.60428907951465, y: 2.2816347393217473 },
                    Position { x: -17.53281923641051, y: 1.8956975865594021 },
                    Position { x: -23.96510511578293, y: 1.924285523801057 }]
            }
        ]);

        // Object tests.
        assert_eq!(level.objects.len(), 8);
        assert_eq!(level.objects, vec![lev::Object {
                position: Position { x: -23.221818747499896, y: -1.3204453531268072 },
                object_type: lev::ObjectType::Killer
            },
            lev::Object {
                position: Position { x: -20.37252715482359, y: -0.3124543521844827 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Normal, animation: 9 }
            },
            lev::Object {
                position: Position { x: -20.3914786548306, y: 0.5277288147929609 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Up, animation: 1 }
            },
            lev::Object {
                position: Position { x: -19.526026821177144, y: 0.36348248139887396 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Right, animation: 5 }
            },
            lev::Object {
                position: Position { x: -21.269564821822065, y: 0.38243398140588436 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Left, animation: 1 }
            },
            lev::Object {
                position: Position { x: -19.55761265452216, y: -0.4387976855645497 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Up, animation: 1 }
            },
            lev::Object {
                position: Position { x: -20.075620321380434, y: -1.2473950191969765 },
                object_type: lev::ObjectType::Exit
            },
            lev::Object {
                position: Position { x: -22.94993115577695, y: 1.5068896484884773 },
                object_type: lev::ObjectType::Player
            }
        ]);

        // Picture tests.
        assert_eq!(level.pictures.len(), 2);
        assert_eq!(level.pictures, vec![lev::Picture {
            name: String::from("barrel"),
            texture: String::new(),
            mask: String::new(),
            position: Position { x: -19.37674118849727, y: 0.895119783101471 },
            distance: 380,
            clip: lev::Clip::Sky
        },
        lev::Picture {
            name: String::new(),
            texture: String::from("stone1"),
            mask: String::from("maskbig"),
            position: Position { x: -24.465394017511894, y: -3.964829547979911 },
            distance: 750,
            clip: lev::Clip::Sky
        }]);

        // TODO: test top10 list
    }

    // TODO: Add more levels to test, including some corrupt ones!

    #[test]
    // Probably redundant, but maybe some new fields are added in the future.
    // Doesn't hurt or impact anything.
    fn rec_default_values () {
        let replay = rec::Replay::new();
        assert_eq!(replay.multi, false);
        assert_eq!(replay.flag_tag, false);
        assert_eq!(replay.link, 0);
        assert_eq!(replay.level, "");
        assert_eq!(replay.frames, vec![]);
        assert_eq!(replay.events, vec![]);
    }

    #[test]
    fn load_valid_replay_1 () {
        let replay = rec::Replay::load_replay("tests/test_1.rec");
        assert_eq!(replay.multi, false);
        assert_eq!(replay.flag_tag, false);
        assert_eq!(replay.link, 2549082363);
        assert_eq!(replay.level, "tutor14.lev");

        // Frames tests.
        assert_eq!(replay.frames.len(), 440);
        assert_eq!(replay.frames[0], rec::Frame {
            bike: Position { x: 34.30250, y: -1.1253119 },
            left_wheel: Position { x: -850, y: -524 },
            right_wheel: Position { x: 849 , y: -524 },
            head: Position { x: 0, y: 439 },
            rotation: 10000,
            left_wheel_rotation: 250,
            right_wheel_rotation: 0,
            throttle: true,
            right: false,
            volume: 5120
        });

        // Event tests.
        assert_eq!(replay.events.len(), 24);
        assert_eq!(replay.events[0], rec::Event {
            time: 1.57728480001688_f64,
            event_type: rec::EventType::VoltRight
         });
        assert_eq!(replay.events[1], rec::Event {
            time: 1.6974048000097273_f64,
            event_type: rec::EventType::Ground { alternative: false }
         });
        assert_eq!(replay.events[11], rec::Event {
            time: 3.9464880000114437_f64,
            event_type: rec::EventType::VoltLeft
         });
        assert_eq!(replay.events[23], rec::Event {
            time: 6.398683200001716_f64,
            event_type: rec::EventType::Touch { index: 3 }
         });
    }
}

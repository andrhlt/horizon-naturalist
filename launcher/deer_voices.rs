fn deer_voices(mob: &Mob) -> std::ops::ControlFlow<[&'static str; 3]> {
    use std::ops::ControlFlow;
    use super::naturalist_bears::{self, BEAR, BLACK_BEAR};
    if mob.kind.id == "naturalist:elephant" {
        return ControlFlow::Break(if mob.is_baby() {
            ["naturalist:entity.elephant.ambient_baby", "naturalist:entity.elephant.hurt_baby", "naturalist:entity.elephant.death_baby"]
        } else {
            ["naturalist:entity.elephant.ambient", "naturalist:entity.elephant.hurt", "naturalist:entity.elephant.death"]
        });
    }
    if mob.kind.id == "naturalist:mammoth" {
        return ControlFlow::Break(if mob.is_baby() {
            ["naturalist:entity.mammoth.ambient_baby", "naturalist:entity.mammoth.hurt_baby", "naturalist:entity.mammoth.death_baby"]
        } else {
            ["naturalist:entity.mammoth.ambient", "naturalist:entity.mammoth.hurt", "naturalist:entity.mammoth.death"]
        });
    }
    if mob.kind.id == "naturalist:whale" {
        return ControlFlow::Break([
            "naturalist:entity.whale.ambient", "naturalist:entity.whale.hurt", "naturalist:entity.whale.death",
        ]);
    }
    if mob.kind.id == "naturalist:great_white_shark" {
        return ControlFlow::Break([
            "naturalist:entity.great_white_shark.ambient",
            "naturalist:entity.great_white_shark.hurt",
            "naturalist:entity.great_white_shark.death",
        ]);
    }
    if mob.kind.id == "naturalist:hippo" {
        return ControlFlow::Break([
            "naturalist:entity.hippo.ambient", "naturalist:entity.hippo.hurt", "naturalist:entity.hippo.death",
        ]);
    }
    if mob.kind.id == "naturalist:alligator" {
        return ControlFlow::Break(if mob.is_baby() {
            ["naturalist:entity.alligator.ambient_baby", "naturalist:entity.alligator.hurt_baby", "naturalist:entity.alligator.death_baby"]
        } else {
            ["naturalist:entity.alligator.ambient", "naturalist:entity.alligator.hurt", "naturalist:entity.alligator.death"]
        });
    }
    if mob.kind.id == BEAR {
        let ambient = if naturalist_bears::state(&mob.ai.nav).sleeping {
            "naturalist:entity.bear.sleep"
        } else if mob.is_baby() { "naturalist:entity.bear.ambient_baby" }
        else { "naturalist:entity.bear.ambient" };
        return ControlFlow::Break(if mob.is_baby() {
            [ambient, "naturalist:entity.bear.hurt_baby", "naturalist:entity.bear.death_baby"]
        } else {
            [ambient, "naturalist:entity.bear.hurt", "naturalist:entity.bear.death"]
        });
    }
    if mob.kind.id == BLACK_BEAR {
        let ambient = if naturalist_bears::state(&mob.ai.nav).sleeping {
            "naturalist:entity.black_bear.sleep"
        } else { "naturalist:entity.black_bear.ambient" };
        return ControlFlow::Break([ambient, "naturalist:entity.black_bear.hurt", "naturalist:entity.black_bear.death"]);
    }
    if mob.kind.id == "naturalist:zebra" {
        return ControlFlow::Break(["naturalist:entity.zebra.ambient", "naturalist:entity.zebra.hurt", "naturalist:entity.zebra.death"]);
    }
    if mob.kind.id == "naturalist:giraffe" {
        return ControlFlow::Break(if mob.is_baby() {
            ["naturalist:entity.giraffe.ambient_baby", "naturalist:entity.giraffe.hurt_baby", "naturalist:entity.giraffe.death_baby"]
        } else {
            ["naturalist:entity.giraffe.ambient", "naturalist:entity.giraffe.hurt", "naturalist:entity.giraffe.death"]
        });
    }
    if mob.kind.id == "naturalist:lion" {
        return ControlFlow::Break(if mob.is_baby() {
            ["naturalist:entity.lion.ambient_baby", "naturalist:entity.lion.hurt_baby", "naturalist:entity.lion.death_baby"]
        } else {
            ["naturalist:entity.lion.ambient", "naturalist:entity.lion.hurt", "naturalist:entity.lion.death"]
        });
    }
    if mob.kind.id == "naturalist:mole" {
        use crate::mobs::goals::naturalist_mole_goals::{self as mole, Phase};
        return ControlFlow::Break([
            if mole::phase(mob) == Phase::Unrolled { "naturalist:entity.mole.ambient" } else { "" },
            "naturalist:entity.mole.hurt",
            "naturalist:entity.mole.death",
        ]);
    }
    if mob.kind.id == "naturalist:rat" {
        let sleeping = super::goals::naturalist_rat_goals::state(mob).sleeping;
        let ambient = if sleeping { "" } else { "naturalist:entity.rat.ambient" };
        return ControlFlow::Break([ambient, "naturalist:entity.rat.hurt", "naturalist:entity.rat.death"]);
    }
    if mob.kind.id == "naturalist:snake" {
        return ControlFlow::Break(["naturalist:entity.snake.ambient", "naturalist:entity.snake.hurt", "naturalist:entity.snake.death"]);
    }
    if mob.kind.id == "naturalist:tiger" {
        return ControlFlow::Break(["naturalist:entity.tiger.ambient", "naturalist:entity.tiger.hurt", "naturalist:entity.tiger.death"]);
    }
    if mob.kind.id == "naturalist:ostrich" {
        return ControlFlow::Break(if mob.is_baby() {
            ["naturalist:entity.ostrich.ambient_baby", "naturalist:entity.ostrich.hurt_baby", "naturalist:entity.ostrich.death_baby"]
        } else {
            ["naturalist:entity.ostrich.ambient", "naturalist:entity.ostrich.hurt", "naturalist:entity.ostrich.death"]
        });
    }
    if mob.kind.id == "naturalist:tortoise" {
        return ControlFlow::Break(if mob.is_baby() {
            ["naturalist:entity.tortoise.ambient_baby", "naturalist:entity.tortoise.hurt_baby", "naturalist:entity.tortoise.death_baby"]
        } else {
            ["naturalist:entity.tortoise.ambient", "naturalist:entity.tortoise.hurt", "naturalist:entity.tortoise.death"]
        });
    }
    if mob.kind.id == "naturalist:rhino" {
        let ambient = if mob.is_baby() {
            "naturalist:entity.rhino.ambient_baby"
        } else {
            "naturalist:entity.rhino.ambient"
        };
        return ControlFlow::Break([ambient, "naturalist:entity.rhino.hurt", "naturalist:entity.rhino.death"]);
    }
    if mob.kind.id == "naturalist:komodo_dragon" {
        return ControlFlow::Break([
            "naturalist:entity.komodo_dragon.ambient", "naturalist:entity.komodo_dragon.hurt", "naturalist:entity.komodo_dragon.death",
        ]);
    }
    if mob.kind.id == "naturalist:lizard_tail" {
        return ControlFlow::Break(["", "entity.generic.hurt", "entity.generic.death"]);
    }
    if mob.kind.id == "naturalist:lizard" {
        return ControlFlow::Break([
            "naturalist:entity.lizard.ambient", "naturalist:entity.lizard.hurt", "naturalist:entity.lizard.death",
        ]);
    }
    if matches!(mob.kind.id.as_str(), "naturalist:desert_scorpion" | "naturalist:jungle_scorpion") {
        return ControlFlow::Break([
            "naturalist:entity.scorpion.ambient", "naturalist:entity.scorpion.hurt", "naturalist:entity.scorpion.death",
        ]);
    }
    if mob.kind.id == "naturalist:capybara" {
        let ambient = if super::naturalist_capybara_sleep::pose(&mob.ai.nav) != 0 {
            "naturalist:entity.capybara.sleep"
        } else {
            "naturalist:entity.capybara.ambient"
        };
        return ControlFlow::Break([ambient, "naturalist:entity.capybara.hurt", "naturalist:entity.capybara.death"]);
    }
    if mob.kind.id == "naturalist:boar" {
        return ControlFlow::Break(if mob.is_baby() {
            ["naturalist:entity.boar.ambient_baby", "naturalist:entity.boar.hurt_baby", "naturalist:entity.boar.death_baby"]
        } else {
            ["naturalist:entity.boar.ambient", "naturalist:entity.boar.hurt", "naturalist:entity.boar.death"]
        });
    }
    if mob.kind.id == "naturalist:turkey" {
        return ControlFlow::Break([
            "naturalist:entity.turkey.ambient", "naturalist:entity.turkey.hurt", "naturalist:entity.turkey.death",
        ]);
    }
    if mob.kind.id == "naturalist:hedgehog" {
        return ControlFlow::Break([
            "naturalist:entity.hedgehog.ambient", "naturalist:entity.hedgehog.hurt",
            "naturalist:entity.hedgehog.death",
        ]);
    }
    if mob.kind.id == "naturalist:ant" {
        return ControlFlow::Break([
            "naturalist:entity.ant.ambient", "naturalist:entity.ant.hurt", "naturalist:entity.ant.death",
        ]);
    }
    if mob.kind.id == "naturalist:snail" {
        return ControlFlow::Break([
            "naturalist:entity.snail.ambient", "naturalist:entity.snail.hurt", "naturalist:entity.snail.crush",
        ]);
    }
    if mob.kind.id == "naturalist:vulture" {
        return ControlFlow::Break([
            "naturalist:entity.vulture.ambient", "naturalist:entity.vulture.hurt", "naturalist:entity.vulture.death",
        ]);
    }
    if mob.kind.id == "naturalist:dragonfly" {
        return ControlFlow::Break(["", "naturalist:entity.dragonfly.hurt", "naturalist:entity.dragonfly.death"]);
    }
    if mob.kind.id == "naturalist:firefly" {
        return ControlFlow::Break([
            "naturalist:entity.firefly.ambient", "naturalist:entity.firefly.hurt", "naturalist:entity.firefly.death",
        ]);
    }
    if mob.kind.id == "naturalist:butterfly" {
        return ControlFlow::Break([
            "naturalist:entity.butterfly.ambient", "naturalist:entity.butterfly.hurt", "",
        ]);
    }
    if mob.kind.id == "naturalist:caterpillar" {
        return ControlFlow::Break(["", "", ""]);
    }
    if mob.kind.id == "naturalist:crab" {
        return ControlFlow::Break(["naturalist:entity.crab.ambient", "naturalist:entity.crab.hurt", "naturalist:entity.crab.death"]);
    }
    if mob.kind.id == "naturalist:bird" {
        return ControlFlow::Break([
            "naturalist:entity.bird.ambient_robin",
            "naturalist:entity.bird.hurt",
            "naturalist:entity.bird.death",
        ]);
    }
    if mob.kind.id == "naturalist:clam" {
        return ControlFlow::Break([
            "naturalist:entity.clam.ambient",
            "naturalist:entity.clam.hurt",
            "naturalist:entity.clam.death",
        ]);
    }
    if matches!(mob.kind.id.as_str(), "naturalist:starfish" | "naturalist:giant_isopod") {
        return ControlFlow::Break(["", "entity.generic.hurt", "entity.generic.death"]);
    }
    if mob.kind.id == "naturalist:anglerfish" {
        return ControlFlow::Break(["naturalist:entity.anglerfish.idle", "naturalist:entity.anglerfish.hurt", "naturalist:entity.anglerfish.death"]);
    }
    if mob.kind.id == "naturalist:ray" {
        return ControlFlow::Break(["naturalist:entity.ray.idle", "naturalist:entity.ray.hurt", "naturalist:entity.ray.death"]);
    }
    if mob.kind.id == "naturalist:jellyfish" {
        return ControlFlow::Break(["naturalist:entity.jellyfish.idle", "naturalist:entity.jellyfish.hurt", "naturalist:entity.jellyfish.death"]);
    }
    if mob.kind.id == "naturalist:blobfish" {
        return ControlFlow::Break(["", "naturalist:entity.blobfish.hurt", "naturalist:entity.blobfish.death"]);
    }
    if mob.kind.id == "naturalist:piranha" {
        return ControlFlow::Break(["naturalist:entity.piranha.idle", "naturalist:entity.piranha.hurt", "naturalist:entity.piranha.death"]);
    }
    if matches!(mob.kind.id.as_str(), "naturalist:bass" | "naturalist:catfish") {
        return ControlFlow::Break(["", "entity.salmon.hurt", "entity.salmon.death"]);
    }
    if mob.kind.id == "naturalist:duck" {
        return ControlFlow::Break(if mob.is_baby() {
            ["naturalist:entity.duck.ambient_baby", "naturalist:entity.duck.hurt_baby", "naturalist:entity.duck.death_baby"]
        } else {
            ["naturalist:entity.duck.ambient", "naturalist:entity.duck.hurt", "naturalist:entity.duck.death"]
        });
    }
    if mob.kind.id != "naturalist:deer" {
        return ControlFlow::Continue(());
    }
    ControlFlow::Break(if mob.is_baby() {
        [
            "naturalist:entity.deer.ambient_baby",
            "naturalist:entity.deer.hurt_baby",
            "naturalist:entity.deer.death_baby",
        ]
    } else {
        [
            "naturalist:entity.deer.ambient",
            "naturalist:entity.deer.hurt",
            "naturalist:entity.deer.death",
        ]
    })
}

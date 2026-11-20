package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.duck.Duck;
import com.crispytwig.naturalist.world.entity.projectile.throwableitemprojectile.ThrownDuckEgg;
import java.util.List;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class DuckHatchProbe {
    public DuckHatchProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("duck_hatch_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var found = level.getEntitiesOfClass(Duck.class, new AABB(0, 90, 0, 16, 96, 16));
            found.forEach(Entity::discard);
            var cases = List.of(0L, 4096L, 4104L);
            var rows = new java.util.ArrayList<String>();
            for (long seed : cases) {
                var egg = NaturalistEntityTypes.DUCK_EGG.get().create(level, EntitySpawnReason.COMMAND);
                if (egg == null) throw new IllegalStateException("Missing egg projectile");
                egg.snapTo(8.5, 94.0, 8.5, 45.0F, 0);
                egg.setDeltaMovement(new Vec3(0.0, -2.0, 0.1));
                egg.getRandom().setSeed(seed);
                level.addFreshEntity(egg);
                egg.tick();
                var ducks = level.getEntitiesOfClass(Duck.class, new AABB(7.5, 92, 7.5, 9.5, 95, 9.5));
                boolean babies = ducks.stream().allMatch(duck -> duck.getAge() == -24000);
                boolean positions = ducks.stream().allMatch(duck -> duck.position().distanceToSqr(egg.position()) < 1.0E-10);
                boolean persistent = ducks.stream().anyMatch(Duck::isPersistenceRequired);
                boolean rotations = ducks.stream().allMatch(duck -> duck.getYRot() == egg.getYRot() && duck.getXRot() == 0);
                rows.add(("{\"seed\":%d,\"hatchlings\":%d,\"baby_age\":%s,"
                    + "\"at_impact\":%s,\"persistent\":%s,\"yaw_at_impact\":%s,\"discarded\":%s}")
                    .formatted(seed, ducks.size(), babies, positions, persistent, rotations, egg.isRemoved()));
                ducks.forEach(Entity::discard);
                egg.discard();
            }
            String report = "{\"cases\":[" + String.join(",", rows) + "]}";
            source.sendSuccess(() -> Component.literal("duck_hatch_fixture " + report), false);
            return 1;
        }));
    }
}

package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.animal.frog.Frog;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class FrogPreyProbe {
    public FrogPreyProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("frog_prey_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var frog = EntityTypes.FROG.create(level, EntitySpawnReason.COMMAND);
            var firefly = NaturalistEntityTypes.FIREFLY.get().create(level, EntitySpawnReason.COMMAND);
            var dragonfly = NaturalistEntityTypes.DRAGONFLY.get().create(level, EntitySpawnReason.COMMAND);
            if (frog == null || firefly == null || dragonfly == null) {
                throw new IllegalStateException("Missing fixture entity");
            }
            frog.snapTo(8.5, 101.0, 8.5, 0.0F, 0.0F);
            try {
                for (LivingEntity insect : new LivingEntity[] {firefly, dragonfly}) {
                    insect.snapTo(8.5, 101.0, 9.0, 0.0F, 0.0F);
                    frog.removeEffect(MobEffects.GLOWING);
                    boolean canEat = Frog.canEat(insect);
                    boolean hit = frog.doHurtTarget(level, insect);
                    var effect = frog.getEffect(MobEffects.GLOWING);
                    int glowing = effect == null ? 0 : effect.getDuration();
                    String line = String.format("%s %s %s %s %d", "frog_prey_fixture",
                            EntityType.getKey(insect.getType()), canEat, hit, glowing);
                    source.sendSuccess(() -> Component.literal(line), false);
                }
                return 1;
            } finally {
                frog.discard();
                firefly.discard();
                dragonfly.discard();
            }
        }));
    }
}

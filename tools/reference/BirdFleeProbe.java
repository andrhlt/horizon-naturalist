package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.mojang.authlib.GameProfile;
import com.mojang.brigadier.arguments.BoolArgumentType;
import com.mojang.brigadier.arguments.IntegerArgumentType;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.level.GameType;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BirdFleeProbe {
    private FakePlayer player;

    public BirdFleeProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("fixture_bird_approach")
            .then(Commands.argument("approach", BoolArgumentType.bool())
                .then(Commands.argument("trial", IntegerArgumentType.integer(0)).executes(ctx -> {
                    var source = ctx.getSource();
                    var level = source.getLevel();
                    if (player != null) {
                        level.removePlayerImmediately(player, Entity.RemovalReason.DISCARDED);
                    }
                    player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "BirdApproach"));
                    boolean approach = BoolArgumentType.getBool(ctx, "approach");
                    player.setGameMode(approach ? GameType.SURVIVAL : GameType.CREATIVE);
                    player.setPermanentlyInvulnerable(false);
                    player.snapTo(12.5, 100.0, 16.5, 90, 0);
                    level.addNewPlayer(player);
                    var bird = NaturalistEntityTypes.BIRD.get().create(level, EntitySpawnReason.COMMAND);
                    if (bird == null) {
                        throw new IllegalStateException("Missing Bird");
                    }
                    bird.snapTo(16.5, 100.0, 16.5, 0, 0);
                    bird.setOnGround(true);
                    bird.addTag("approach_bird");
                    bird.addTag("approach_bird_" + IntegerArgumentType.getInteger(ctx, "trial"));
                    level.addFreshEntity(bird);
                    source.sendSuccess(() -> Component.literal("fixture bird approach ready"), false);
                    return 1;
                }))));
    }
}

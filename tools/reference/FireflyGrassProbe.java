package naturalist.fixture;

import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.particles.BlockParticleOption;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.PlayLevelSoundEvent;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class FireflyGrassProbe {
    private final ArrayList<Map<String, Object>> hiding = new ArrayList<>();
    private final ArrayList<Map<String, Object>> sounds = new ArrayList<>();
    private ServerPlayer observer;

    public FireflyGrassProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
        NeoForge.EVENT_BUS.addListener(this::sound);
    }

    private void sound(PlayLevelSoundEvent.AtPosition sound) {
        var id = sound.getSound().value().location().toString();
        if (observer != null && id.equals("naturalist:entity.firefly.hide")) {
            var position = sound.getPosition();
            sounds.add(Map.of("id", id, "source", sound.getSource().ordinal(),
                              "at", new double[]{position.x(), position.y(), position.z()},
                              "volume", sound.getNewVolume(), "pitch", sound.getNewPitch()));
        }
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("firefly_grass_fixture")
            .then(Commands.literal("start").executes(ctx -> {
                var source = ctx.getSource();
                var level = source.getLevel();
                var profile = new GameProfile(UUID.randomUUID(), "GrassWitness");
                var server = source.getServer();
                observer = new ServerPlayer(server, level, profile, ClientInformation.createDefault());
                observer.connection = new ServerGamePacketListenerImpl(server,
                    new Connection(PacketFlow.SERVERBOUND), observer, CommonListenerCookie.createInitial(profile, false)) {
                    @Override
                    public void send(Packet<?> packet) {
                        if (packet instanceof ClientboundLevelParticlesPacket burst
                            && burst.particle() instanceof BlockParticleOption block
                            && block.getState().is(Blocks.SHORT_GRASS)
                            && burst.count() == 50) {
                            Map<String, Object> row = new LinkedHashMap<>();
                            row.put("at", new double[]{burst.x(), burst.y(), burst.z()});
                            row.put("spread", new float[]{burst.xDist(), burst.yDist(), burst.zDist()});
                            row.put("speed", new float[]{burst.xMaxSpeed(), burst.yMaxSpeed(), burst.zMaxSpeed()});
                            row.put("count", burst.count());
                            row.put("tick", level.getGameTime());
                            hiding.add(row);
                        }
                    }
                };
                observer.snapTo(8.5, 104, 7.5, 0, 0);
                level.addNewPlayer(observer);
                observer.setGameMode(GameType.SPECTATOR);
                hiding.clear();
                sounds.clear();
                return 1;
            })).then(Commands.literal("result").executes(ctx -> {
                var source = ctx.getSource();
                var result = new Gson().toJson(Map.of("hiding_packets", hiding, "sound_events", sounds));
                source.sendSuccess(() -> Component.literal("firefly_grass_packets " + result), false);
                return 1;
            })).then(Commands.literal("light").executes(ctx -> {
                var source = ctx.getSource();
                var level = source.getLevel();
                var row = Map.of("darken", level.getSkyDarken(),
                                 "bright", level.isBrightOutside(),
                                 "raw", level.getMaxLocalRawBrightness(new BlockPos(5, 101, 5)));
                source.sendSuccess(() -> Component.literal("firefly_grass_light " + new Gson().toJson(row)), false);
                return 1;
            })).then(Commands.literal("stop").executes(ctx -> {
                if (observer != null) {
                    ctx.getSource().getLevel().removePlayerImmediately(observer, Entity.RemovalReason.DISCARDED);
                    observer = null;
                }
                return 1;
            })));
    }
}

package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.util.ProblemReporter;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.storage.TagValueOutput;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class PachydermPlayerHerdProbe {
    public PachydermPlayerHerdProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_pachyderm_player_herd").executes(context -> {
            var source = context.getSource();
            ServerLevel level = source.getLevel();
            var records = new LinkedHashMap<String, Object>();
            var first = player(level, "ElephantFirst", 8.5, 6.5);
            var second = player(level, "ElephantSecond", 11.1, 8.5);
            try {
                for (String species : List.of("elephant", "mammoth")) {
                    for (var attacker : List.of(first, second)) {
                        var animal = species.equals("mammoth")
                            ? NaturalistEntityTypes.MAMMOTH.get().create(level, EntitySpawnReason.COMMAND)
                            : NaturalistEntityTypes.ELEPHANT.get().create(level, EntitySpawnReason.COMMAND);
                        if (animal == null) {
                            throw new IllegalStateException(species);
                        }
                        var child = species.equals("mammoth")
                            ? NaturalistEntityTypes.MAMMOTH.get().create(level, EntitySpawnReason.COMMAND)
                            : NaturalistEntityTypes.ELEPHANT.get().create(level, EntitySpawnReason.COMMAND);
                        if (child == null) {
                            throw new IllegalStateException(species);
                        }
                        child.snapTo(7.5, 102, 8.5, 0, 0);
                        child.setAge(-10000);
                        child.setOnGround(true);
                        level.addFreshEntity(child);
                        var bystander = attacker == first ? second : first;
                        attacker.snapTo(7.5, 102, 6.5, 0, 0);
                        bystander.snapTo(13.8, 102, 6.5, 0, 0);
                        animal.snapTo(12.5, 102.0, 8.5, 0.0F, 0.0F);
                        animal.setOnGround(true);
                        level.addFreshEntity(animal);
                        for (int tick = 0; tick < 8; tick++) {
                            level.tickNonPassenger(child);
                            level.tickNonPassenger(animal);
                        }
                        var freshEnd = animal.getPersistentAngerEndTime();
                        float before = child.getHealth();
                        float adultBefore = animal.getHealth();
                        boolean applied = child.hurtServer(level, child.damageSources().playerAttack(attacker), 1.0f);
                        for (int tick = 0; tick < 8; tick++) {
                            level.tickNonPassenger(child);
                            level.tickNonPassenger(animal);
                        }
                        long end = animal.getPersistentAngerEndTime();
                        var saved = TagValueOutput.createWithContext(ProblemReporter.DISCARDING, level.registryAccess());
                        animal.addPersistentAngerSaveData(saved);
                        var tag = saved.buildResult();
                        var actor = animal.getPersistentAngerTarget();
                        var exactTarget = actor != null && actor.matches(attacker);
                        var exactSaved = tag.toString().contains("angry_at")
                            && tag.toString().contains("anger_end_time:" + end + "L");
                        attacker.setGameMode(GameType.SPECTATOR);
                        for (int tick = 0; tick < 8; tick++) {
                            level.tickNonPassenger(child);
                            level.tickNonPassenger(animal);
                        }
                        records.put(species + (attacker == first ? "_first" : "_second"), Map.of(
                            "applied", applied,
                            "health_decreased", child.getHealth() < before,
                            "adult_uninjured", animal.getHealth() == adultBefore,
                            "fresh_end", freshEnd,
                            "attacker_match", exactTarget,
                            "stored_end", end,
                            "store_has_anger", exactSaved,
                            "left", end - level.getGameTime(),
                            "calm_end", animal.getPersistentAngerEndTime(),
                            "calm_target_absent", animal.getPersistentAngerTarget() == null
                        ));
                        attacker.setGameMode(GameType.SURVIVAL);
                        child.discard();
                        animal.discard();
                    }
                }
            } finally {
                first.discard();
                second.discard();
            }
            source.sendSuccess(() -> Component.literal("pachyderm_player_herd " + new Gson().toJson(records)), false);
            return 1;
        }));
    }

    private static ServerPlayer player(ServerLevel level, String name, double x, double z) {
        var profile = new GameProfile(UUID.randomUUID(), name);
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(x, 102, z, 0, 0);
        level.addNewPlayer(player);
        player.setGameMode(GameType.SURVIVAL);
        return player;
    }
}

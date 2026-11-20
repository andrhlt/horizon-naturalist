package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.deer.Deer;
import com.google.gson.GsonBuilder;
import com.mojang.brigadier.arguments.LongArgumentType;
import java.util.LinkedHashMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.entity.SpawnerBlockEntity;
import com.mojang.authlib.GameProfile;
import java.util.UUID;
import net.minecraft.network.Connection;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.phys.AABB;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class DeerSpawnerProbe {
    public DeerSpawnerProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_deer_entry")
                .then(Commands.literal("seed")
                        .then(Commands.argument("seed", LongArgumentType.longArg()).executes(ctx -> {
                            var level = ctx.getSource().getLevel();
                            level.getRandom().setSeed(LongArgumentType.getLong(ctx, "seed"));
                            ctx.getSource().sendSuccess(() -> Component.literal("Deer entry seeded"), false);
                            return 1;
                        })))
                .then(Commands.literal("spawner").executes(ctx -> {
                    var source = ctx.getSource();
                    var level = source.getLevel();
                    var position = new BlockPos(8, 100, 8);
                    var spawner = level.getBlockEntity(position);
                    if (!(spawner instanceof SpawnerBlockEntity block)) return 0;
                    var profile = new GameProfile(UUID.randomUUID(), "DeerSpawnerFixture");
                    var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
                    player.connection = new ServerGamePacketListenerImpl(level.getServer(),
                            new Connection(PacketFlow.SERVERBOUND), player,
                            CommonListenerCookie.createInitial(profile, false));
                    player.snapTo(8.5, 100, 15.5, 0, 0);
                    level.addNewPlayer(player);
                    player.connection.markClientLoaded();
                    player.setGameMode(GameType.SURVIVAL);
                    try {
                        var isNear = level.hasNearbyAlivePlayer(8.5, 100.5, 8.5, 16);
                        var before = block.saveWithoutMetadata(level.registryAccess()).toString();
                        SpawnerBlockEntity.serverTick(level, position, level.getBlockState(position), block);
                        var after = block.saveWithoutMetadata(level.registryAccess()).toString();
                        source.sendSuccess(() -> Component.literal("Spawner probe near="+isNear+" enabled="+level.isSpawnerBlockEnabled()+" before="+before+" after="+after), false);
                    } finally {
                        level.removePlayerImmediately(player, Entity.RemovalReason.DISCARDED);
                    }
                    return 1;
                }))
                .then(Commands.literal("observe").executes(ctx -> {
                    var source = ctx.getSource();
                    var level = source.getLevel();
                    var region = new AABB(2, 95, 2, 15, 106, 15);
                    var deer = level.getEntitiesOfClass(Deer.class, region).stream().findFirst().orElse(null);
                    if (deer == null) {
                        source.sendFailure(Component.literal("Deer entry has no actor"));
                        return 0;
                    }
                    var modifier = deer.getAttribute(Attributes.FOLLOW_RANGE)
                            .getModifier(Identifier.withDefaultNamespace("random_spawn_bonus"));
                    var row = new LinkedHashMap<String, Object>();
                    row.put("pos", new double[]{deer.getX(), deer.getY(), deer.getZ()});
                    row.put("variant", deer.getVariantString());
                    row.put("follow_bonus", modifier == null ? null : modifier.amount());
                    row.put("left_handed", deer.isLeftHanded());
                    row.put("age", deer.getAge());
                    row.put("yaw", deer.getYRot());
                    row.put("next", level.getRandom().nextLong());
                    deer.discard();
                    var text = new GsonBuilder().serializeNulls().create().toJson(row);
                    source.sendSuccess(() -> Component.literal("Naturalist Deer entry " + text), false);
                    return 1;
                })));
    }
}

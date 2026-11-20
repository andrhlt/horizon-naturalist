package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.MobPart;
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
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.level.GameType;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class MultipartContactProbe {
    public MultipartContactProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("multipart_contact_fixture").executes(command -> {
            var source = command.getSource();
            var level = source.getLevel();
            var profile = new GameProfile(UUID.randomUUID(), "PartsContact");
            var actor = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
            actor.connection = new ServerGamePacketListenerImpl(level.getServer(),
                new Connection(PacketFlow.SERVERBOUND), actor, CommonListenerCookie.createInitial(profile, false));
            actor.snapTo(8.85, 92, 12.6, 0, 0);
            level.addNewPlayer(actor);
            actor.connection.markClientLoaded();
            actor.setGameMode(GameType.SURVIVAL);
            var result = new LinkedHashMap<String, Object>();
            try {
                var shark = NaturalistEntityTypes.GREAT_WHITE_SHARK.get().create(level, EntitySpawnReason.COMMAND);
                var whale = NaturalistEntityTypes.WHALE.get().create(level, EntitySpawnReason.COMMAND);
                result.put("great_white_shark", sample(shark, actor, 12.6));
                result.put("whale", sample(whale, actor, 14.7));
            } catch (ReflectiveOperationException failure) {
                throw new IllegalStateException(failure);
            } finally {
                level.removePlayerImmediately(actor, Entity.RemovalReason.DISCARDED);
            }
            source.sendSuccess(() -> Component.literal("multipart_contact_fixture " + new Gson().toJson(result)), false);
            return 1;
        }));
    }

    private static Object sample(Mob mob, ServerPlayer actor, double z) throws ReflectiveOperationException {
        mob.snapTo(8.5, 92, 10.5, 0, 0);
        actor.snapTo(8.85, 92, z, 0, 0);
        actor.setDeltaMovement(Vec3.ZERO);
        var position = mob.getClass().getDeclaredMethod("positionParts");
        position.setAccessible(true);
        position.invoke(mob);
        var parts = mob.getClass().getDeclaredField("parts");
        parts.setAccessible(true);
        var bodies = (MobPart[]) parts.get(mob);
        var contacts = java.util.Arrays.stream(bodies).filter(part -> part.getBoundingBox().intersects(actor.getBoundingBox())).count();
        int seen = 0;
        for (var part : bodies) {
            seen += mob.level().getEntities(part, part.getBoundingBox(), e -> e == actor).size();
        }
        MobPart.pushEntities(mob, bodies);
        var delta = actor.getDeltaMovement();
        actor.setDeltaMovement(Vec3.ZERO);
        for (var part : bodies) {
            if (part.getBoundingBox().intersects(actor.getBoundingBox())) {
                part.push(actor);
            }
        }
        var direct = actor.getDeltaMovement();
        return Map.of("body", mob.getBoundingBox().intersects(actor.getBoundingBox()),
            "parts", contacts, "pushable", actor.isPushable(), "seen", seen,
            "delta", List.of(delta.x, delta.y, delta.z), "direct", List.of(direct.x, direct.y, direct.z));
    }
}

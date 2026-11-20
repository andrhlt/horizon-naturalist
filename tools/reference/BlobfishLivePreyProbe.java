package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.fish.Blobfish;
import com.crispytwig.naturalist.world.entity.animal.crab.Crab;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.animal.fish.Cod;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BlobfishLivePreyProbe {
    private static final Gson GSON = new Gson();
    private Blobfish blob;
    private Crab crab;
    private Cod cod;

    public BlobfishLivePreyProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("blobfish_live_prey_fixture")
            .then(Commands.literal("cod").executes(ctx -> {
                var level = ctx.getSource().getLevel();
                blob = NaturalistEntityTypes.BLOBFISH.get().create(level, EntitySpawnReason.COMMAND);
                cod = EntityTypes.COD.create(level, EntitySpawnReason.COMMAND);
                blob.snapTo(8.5, 29.3, 8.5, 0, 0);
                cod.snapTo(8.5, 29.3, 9.5, 0, 0);
                blob.setNoGravity(true);
                cod.setNoAi(true);
                cod.setNoGravity(true);
                blob.setPersistenceRequired();
                cod.setPersistenceRequired();
                level.addFreshEntity(blob);
                level.addFreshEntity(cod);
                ctx.getSource().sendSuccess(() -> Component.literal("Blobfish and Cod added."), false);
                return 1;
            }))
            .then(Commands.literal("crab").executes(ctx -> {
                var level = ctx.getSource().getLevel();
                crab = NaturalistEntityTypes.CRAB.get().create(level, EntitySpawnReason.COMMAND);
                crab.snapTo(8.9, 29.3, 8.5, 0, 0);
                crab.setNoAi(true);
                crab.setNoGravity(true);
                crab.setPersistenceRequired();
                blob.snapTo(8.5, 29.3, 8.5, 0, 0);
                blob.setDeltaMovement(Vec3.ZERO);
                blob.getNavigation().stop();
                level.addFreshEntity(crab);
                ctx.getSource().sendSuccess(() -> Component.literal("Crab added."), false);
                return 1;
            }))
            .then(Commands.literal("observe").executes(ctx -> {
                var reply = new LinkedHashMap<String, Object>();
                reply.put("vanilla_cod_target", blob.getTarget() == cod);
                reply.put("naturalist_crab_target", crab != null && blob.getTarget() == crab);
                reply.put("naturalist_crab_hit", crab != null && crab.getHealth() < crab.getMaxHealth());
                ctx.getSource().sendSuccess(() -> Component.literal("blobfish_live_prey_fixture " + GSON.toJson(reply)), false);
                return 1;
            })));
    }
}

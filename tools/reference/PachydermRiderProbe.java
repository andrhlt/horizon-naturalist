package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class PachydermRiderProbe {
    public PachydermRiderProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_pachyderm_rider").executes(context -> {
            var source = context.getSource();
            ServerLevel level = source.getLevel();
            var records = new LinkedHashMap<String, Object>();
            for (String species : List.of("elephant", "mammoth")) {
                for (String mode : List.of("none", "still_mob", "steering_mob", "item")) {
                    var animal = species.equals("mammoth")
                        ? NaturalistEntityTypes.MAMMOTH.get().create(level, EntitySpawnReason.COMMAND)
                        : NaturalistEntityTypes.ELEPHANT.get().create(level, EntitySpawnReason.COMMAND);
                    if (animal == null) {
                        throw new IllegalStateException(species);
                    }
                    animal.snapTo(8.5, 102.0, 8.5, 0.0F, 0.0F);
                    animal.setOnGround(true);
                    animal.setSpeed(0.2F);
                    Entity passenger = null;
                    if (mode.endsWith("_mob")) {
                        var chicken = EntityTypes.CHICKEN.create(level, EntitySpawnReason.COMMAND);
                        if (chicken == null) {
                            throw new IllegalStateException("Chicken creation failed");
                        }
                        chicken.snapTo(8.5, 102.0, 8.5, 90.0F, 30.0F);
                        chicken.xxa = mode.equals("steering_mob") ? 0.5F : 0.0F;
                        chicken.zza = mode.equals("steering_mob") ? 1.0F : 0.0F;
                        passenger = chicken;
                    } else if (mode.equals("item")) {
                        passenger = new ItemEntity(level, 8.5, 102.0, 8.5, new ItemStack(Items.APPLE));
                    }
                    if (passenger != null && !passenger.startRiding(animal, true, false)) {
                        throw new IllegalStateException("Fixture passenger did not mount: " + mode);
                    }
                    animal.travel(new Vec3(0.0, 0.0, 1.0));
                    var motion = animal.getDeltaMovement();
                    records.put(species + "_" + mode, Map.of(
                        "controller", animal.getControllingPassenger() != null,
                        "position", List.of(animal.getX(), animal.getY(), animal.getZ()),
                        "delta", List.of(motion.x, motion.y, motion.z),
                        "rotation", List.of(animal.getYRot(), animal.getXRot(), animal.yBodyRot),
                        "speed", animal.getSpeed()
                    ));
                    if (passenger != null) {
                        passenger.discard();
                    }
                    animal.discard();
                }
            }
            String json = new Gson().toJson(records);
            source.sendSuccess(() -> Component.literal("pachyderm_rider " + json), false);
            return 1;
        }));
    }
}

package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.MobPart;
import com.google.gson.Gson;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.List;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Mob;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class MultipartRotationProbe {
    private static final float[][] ROTATIONS = {
        {0, 0, 0, 0}, {32.5F, 14.25F, 0.31F, 0.62F},
        {-53.8F, -9.75F, 0.24F, 0.13F}, {143.6F, 28.2F, 0.75F, 0.10F},
        {-179.4F, -40F, 0.02F, 0.9F},
    };

    public MultipartRotationProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("multipart_rotation_fixture").executes(command -> {
            var source = command.getSource();
            var level = source.getLevel();
            var result = new LinkedHashMap<String, Object>();
            try {
                for (int id = 0; id < ROTATIONS.length; id++) {
                    float[] state = ROTATIONS[id];
                    var shark = NaturalistEntityTypes.GREAT_WHITE_SHARK.get().create(level, EntitySpawnReason.COMMAND);
                    var adult = NaturalistEntityTypes.WHALE.get().create(level, EntitySpawnReason.COMMAND);
                    var calf = NaturalistEntityTypes.WHALE.get().create(level, EntitySpawnReason.COMMAND);
                    calf.setAge(-24000);
                    result.put(Integer.toString(id), List.of(
                        sample(shark, state, false), sample(adult, state, true), sample(calf, state, true)));
                }
            } catch (ReflectiveOperationException failure) {
                throw new IllegalStateException(failure);
            }
            source.sendSuccess(() -> Component.literal("multipart_rotation_fixture " + new Gson().toJson(result)), false);
            return 1;
        }));
    }

    private static List<List<Double>> sample(Mob mob, float[] state, boolean whale) throws ReflectiveOperationException {
        mob.snapTo(32.375D, 100.625D, 32.875D, state[0], 0.0F);
        var mobType = mob.getClass();
        var chain = mobType.getDeclaredField("chain");
        chain.setAccessible(true);
        var bodyChain = chain.get(mob);
        var renderYaw = bodyChain.getClass().getDeclaredField("renderYaw");
        renderYaw.setAccessible(true);
        renderYaw.setFloat(bodyChain, state[0]);
        var pitch = mobType.getField("xBodyRot");
        pitch.setFloat(mob, state[1]);
        if (whale) {
            var front = mobType.getDeclaredField("frontDroop");
            var back = mobType.getDeclaredField("backDroop");
            front.setAccessible(true);
            back.setAccessible(true);
            front.setFloat(mob, state[2]);
            back.setFloat(mob, state[3]);
        }
        var position = mobType.getDeclaredMethod("positionParts");
        position.setAccessible(true);
        position.invoke(mob);
        var parts = mobType.getDeclaredField("parts");
        parts.setAccessible(true);
        return Arrays.stream((MobPart[]) parts.get(mob)).map(part -> {
            var box = part.getBoundingBox();
            return List.of(box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ);
        }).toList();
    }
}

package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.google.gson.Gson;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Proxy;
import java.util.ArrayList;
import java.util.List;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.SpawnGroupData;
import net.minecraft.resources.Identifier;
import net.minecraft.util.RandomSource;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.level.ServerLevelAccessor;
import net.minecraft.world.level.levelgen.LegacyRandomSource;
import net.minecraft.world.level.levelgen.XoroshiroRandomSource;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class DeerLevelSpawnProbe {
    public DeerLevelSpawnProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private record Member(String variant, int age, double follow_bonus, boolean left_handed) {}
    private record Group(long seed, String stream, List<Member> members, long next) {}
    private record Fixture(String biome, String random, List<Group> groups) {}

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_deer_level_spawn_fixture").executes(ctx -> {
            var level = ctx.getSource().getLevel();
            var position = new BlockPos(8, 100, 8);
            var difficulty = level.getCurrentDifficultyAt(position);
            var sample = new ArrayList<Group>();
            for (String stream : List.of("legacy", "xoroshiro")) {
                for (int i = 0; i < 32; i++) {
                    long seed = 7198391L + i * 15485863L;
                    RandomSource random = stream.equals("legacy")
                            ? new LegacyRandomSource(seed) : new XoroshiroRandomSource(seed);
                    var region = (ServerLevelAccessor) Proxy.newProxyInstance(
                            ServerLevelAccessor.class.getClassLoader(),
                            new Class<?>[] {ServerLevelAccessor.class},
                            (proxy, method, args) -> {
                                if (method.getName().equals("getRandom")) return random;
                                try {
                                    return method.invoke(level, args);
                                } catch (InvocationTargetException error) {
                                    throw error.getCause();
                                }
                            });
                    SpawnGroupData group = null;
                    var members = new ArrayList<Member>();
                    for (int j = 0; j < 5; j++) {
                        var deer = NaturalistEntityTypes.DEER.get().create(level, EntitySpawnReason.CHUNK_GENERATION);
                        deer.snapTo(8.5, 100, 8.5, 0, 0);
                        group = deer.finalizeSpawn(region, difficulty, EntitySpawnReason.CHUNK_GENERATION, group);
                        var follow = deer.getAttribute(Attributes.FOLLOW_RANGE)
                                .getModifier(Identifier.withDefaultNamespace("random_spawn_bonus")).amount();
                        members.add(new Member(deer.getVariantString(), deer.getAge(), follow, deer.isLeftHanded()));
                        deer.discard();
                    }
                    sample.add(new Group(seed, stream, members, random.nextLong()));
                }
            }
            var key = level.getBiome(position).unwrapKey().orElseThrow().identifier().toString();
            var result = new Fixture(key, "region_injected", sample);
            var data = new Gson().toJson(result);
            ctx.getSource().sendSuccess(() -> Component.literal("naturalist_deer_level_spawn_fixture " + data), false);
            return 1;
        }));
    }
}

package naturalist.fixture;

import com.crispytwig.naturalist.world.entity.animal.clam.Clam;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.TreeSet;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.AgeableMob;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ExperienceOrb;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.phys.AABB;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class MobExperienceProbe {
    private static final AABB AREA = new AABB(6, 145, 6, 11, 155, 11);

    public MobExperienceProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_xp").executes(context -> {
            var source = context.getSource();
            var result = new LinkedHashMap<String, Map<String, TreeSet<Integer>>>();
            for (var type : BuiltInRegistries.ENTITY_TYPE) {
                var id = BuiltInRegistries.ENTITY_TYPE.getKey(type);
                if (!id.getNamespace().equals("naturalist")
                        || !(type.create(source.getLevel(), EntitySpawnReason.COMMAND) instanceof Mob)) {
                    continue;
                }
                String species = id.getPath();
                var ages = new LinkedHashMap<String, TreeSet<Integer>>();
                for (int age : new int[]{0, -24000}) {
                    var rolls = new TreeSet<Integer>();
                    for (int i = 0; i < 30; i++) {
                        rolls.add(kill(source.getLevel(), "naturalist:" + species, age));
                    }
                    ages.put(age == 0 ? "adult" : "juvenile_tag", rolls);
                }
                result.put(species, ages);
            }
            source.sendSuccess(() -> Component.literal("naturalist_fixture_xp " + new Gson().toJson(result)), false);
            return 1;
        }));
    }

    private static int kill(ServerLevel level, String id, int age) {
        var player = FakePlayerFactory.getMinecraft(level);
        var type = BuiltInRegistries.ENTITY_TYPE.getValue(Identifier.parse(id));
        if (!(type.create(level, EntitySpawnReason.COMMAND) instanceof Mob mob)) {
            throw new IllegalStateException(id);
        }
        mob.snapTo(8.5, 150, 8.5, 0, 0);
        if (mob instanceof AgeableMob ageable) {
            ageable.setAge(age);
        }
        if (mob instanceof Clam clam) {
            clam.setOpen(true);
        }
        level.addFreshEntity(mob);
        if (!mob.hurtServer(level, mob.damageSources().playerAttack(player), 200.0f) || mob.isAlive()) {
            throw new IllegalStateException("Fixture failed to kill " + id);
        }
        var orbs = level.getEntitiesOfClass(ExperienceOrb.class, AREA);
        int experience = orbs.stream().mapToInt(ExperienceOrb::getValue).sum();
        orbs.forEach(Entity::discard);
        mob.discard();
        return experience;
    }
}

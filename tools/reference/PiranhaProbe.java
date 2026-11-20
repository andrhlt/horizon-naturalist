package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.fish.Piranha;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.goal.FollowFlockLeaderGoal;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;
import java.util.stream.IntStream;

@Mod("naturalist_fixture")
public final class PiranhaProbe {
    public PiranhaProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("piranha_fixture").executes(ctx -> {
            var source = ctx.getSource();
            ServerLevel level = source.getLevel();
            Piranha leader = fish(level);
            Piranha follower = fish(level);
            follower.startFollowing(leader);
            var cod = EntityTypes.COD.create(level, EntitySpawnReason.COMMAND);
            var salmon = EntityTypes.SALMON.create(level, EntitySpawnReason.COMMAND);
            var damage = level.damageSources().mobAttack(cod);
            boolean landed = follower.hurtServer(level, damage, 1.0F);
            boolean selfTargets = follower.getTarget() == cod;
            boolean leaderRetaliates = leader.getTarget() == cod;
            leader.setTarget(salmon);
            follower.aiStep();
            boolean followsRetarget = follower.getTarget() == salmon;
            leader.setTarget(null);
            follower.aiStep();
            boolean followsClear = follower.getTarget() == null;

            Piranha capLeader = fish(level);
            var cohort = IntStream.range(0, 25).mapToObj(i -> fish(level)).toList();
            capLeader.addFollowers(cohort.stream());
            long attached = cohort.stream().filter(Piranha::isFollower).count();

            var live = IntStream.range(0, 25).mapToObj(i -> fish(level)).toList();
            for (var recruit : live) {
                recruit.snapTo(14.1, 102.25, 8.5, 0, 0);
                level.addFreshEntity(recruit);
            }
            var liveLeader = fish(level);
            liveLeader.snapTo(16.25, 102.25, 8.5, 0, 0);
            level.addFreshEntity(liveLeader);
            var goal = new FollowFlockLeaderGoal(liveLeader);
            for (int delay = 0; delay < 400 && !liveLeader.hasFollowers(); delay++) {
                goal.canUse();
            }
            long firstRecruitment = live.stream().filter(Piranha::isFollower).count();
            live.forEach(Piranha::discard);
            liveLeader.discard();
            int sea = level.getSeaLevel();
            boolean deep = Piranha.checkPiranhaSpawnRules(NaturalistEntityTypes.PIRANHA.get(), level,
                EntitySpawnReason.NATURAL, new BlockPos(8, sea - 30, 8), level.getRandom());
            boolean high = Piranha.checkPiranhaSpawnRules(NaturalistEntityTypes.PIRANHA.get(), level,
                EntitySpawnReason.NATURAL, new BlockPos(8, sea + 1, 8), level.getRandom());
            boolean dry = Piranha.checkPiranhaSpawnRules(NaturalistEntityTypes.PIRANHA.get(), level,
                EntitySpawnReason.NATURAL, new BlockPos(7, sea - 30, 8), level.getRandom());
            String result = """
                {"max_health":%s,"attack_damage":%s,"movement_speed":%s,"variant":"%s",\
                "attached_at_capacity":%d,"first_recruitment":%d,"hurt_landed":%s,"follower_retaliates":%s,\
                "leader_retaliates":%s,"follower_retargets":%s,"follower_clears_target":%s,\
                "deep_water":%s,"above_sea_level":%s,"dry":%s}
                """.strip().formatted(leader.getAttributeBaseValue(Attributes.MAX_HEALTH),
                    leader.getAttributeBaseValue(Attributes.ATTACK_DAMAGE),
                    leader.getAttributeBaseValue(Attributes.MOVEMENT_SPEED), leader.getVariantString(),
                    attached, firstRecruitment, landed, selfTargets, leaderRetaliates, followsRetarget, followsClear, deep, high, dry);
            source.sendSuccess(() -> Component.literal("piranha_fixture " + result), false);
            return 1;
        }));
    }

    private static Piranha fish(ServerLevel level) {
        var fish = NaturalistEntityTypes.PIRANHA.get().create(level, EntitySpawnReason.COMMAND);
        fish.snapTo(8.5, 90.0, 8.5, 0, 0);
        fish.setNoAi(true);
        return fish;
    }
}

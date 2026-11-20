package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.bird.Bird;
import com.mojang.authlib.GameProfile;
import java.util.Locale;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Pose;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BirdPerchProbe {
    public BirdPerchProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("bird_perch_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "PerchFixture"));
            player.snapTo(8.5, 93.0, 8.5, 30.0F, 3.0F);
            player.setYBodyRot(22.0F);
            player.setYHeadRot(45.0F);
            level.addNewPlayer(player);
            var bird = NaturalistEntityTypes.BIRD.get().create(level, EntitySpawnReason.COMMAND);
            var other = NaturalistEntityTypes.BIRD.get().create(level, EntitySpawnReason.COMMAND);
            if (bird == null || other == null) throw new IllegalStateException("Missing fixture bird");
            for (Bird specimen : new Bird[] {bird, other}) {
                specimen.snapTo(8.6, 93.0, 8.5, 0.0F, 0.0F);
                specimen.setOwner(player);
                specimen.setTame(true, false);
                level.addFreshEntity(specimen);
            }
            try {
                var type = Class.forName(Bird.class.getName() + "$MountOnOwnersHeadGoal");
                var form = type.getDeclaredConstructor(Bird.class);
                form.setAccessible(true);
                var goal = form.newInstance(bird);
                var otherGoal = form.newInstance(other);
                var canUse = type.getDeclaredMethod("canUse");
                var canContinue = type.getDeclaredMethod("canContinueToUse");
                var start = type.getDeclaredMethod("start");
                canUse.setAccessible(true);
                canContinue.setAccessible(true);
                start.setAccessible(true);
                bird.setTame(false, false);
                boolean rejectsUntamed = !(boolean)canUse.invoke(goal);
                bird.setTame(true, false);
                bird.setOrderedToSit(true);
                boolean rejectsSitting = !(boolean)canUse.invoke(goal);
                bird.setOrderedToSit(false);
                player.getAbilities().flying = true;
                boolean rejectsFlying = !(boolean)canUse.invoke(goal);
                player.getAbilities().flying = false;
                bird.snapTo(10.5, 93.0, 8.5, 0.0F, 0.0F);
                boolean rejectsOutOfContact = !(boolean)canUse.invoke(goal);
                bird.snapTo(8.6, 93.0, 8.5, 0.0F, 0.0F);
                boolean contactAllowed = (boolean)canUse.invoke(goal);
                if (!contactAllowed) throw new IllegalStateException("Owner cannot be resolved");
                bird.getNavigation().moveTo(14.5, 93.0, 8.5, 1.0);
                boolean searching = !bird.getNavigation().isDone();
                start.invoke(goal);
                boolean ordinaryMountBlocked = bird.getVehicle() == null && player.getPassengers().isEmpty();
                boolean forcedMountBlocked = !bird.startRiding(player, true, true);
                boolean pathStopped = searching && bird.getNavigation().isDone();
                boolean secondStillEligible = (boolean)canUse.invoke(otherGoal);
                boolean singleUse = !(boolean)canContinue.invoke(goal);
                double anchorHeight = player.getPassengerRidingPosition(bird).y - player.getY();
                double attachment = bird.getVehicleAttachmentPoint(player).y;
                player.setPose(Pose.CROUCHING);
                player.setShiftKeyDown(true);
                boolean rejectsCrouch = !(boolean)canUse.invoke(otherGoal);
                String report = String.format(Locale.ROOT,
                    "{\"rejects_untamed\":%s,\"rejects_sitting\":%s,\"rejects_flying\":%s,"
                        + "\"rejects_out_of_contact\":%s,\"contact_allowed\":%s,\"player_serializable\":%s,"
                        + "\"ordinary_mount_blocked\":%s,\"forced_mount_blocked\":%s,"
                        + "\"stops_existing_navigation\":%s,\"second_still_eligible\":%s,"
                        + "\"mount_goal_single_use\":%s,\"standing_anchor_y\":%.4f,"
                        + "\"attachment_y\":%.4f,\"rejects_crouch\":%s}",
                    rejectsUntamed, rejectsSitting, rejectsFlying, rejectsOutOfContact,
                    contactAllowed, player.getType().canSerialize(), ordinaryMountBlocked, forcedMountBlocked,
                    pathStopped, secondStillEligible, singleUse, anchorHeight, attachment, rejectsCrouch);
                source.sendSuccess(() -> Component.literal("bird_perch_fixture " + report), false);
                return 1;
            } catch (ReflectiveOperationException error) {
                throw new IllegalStateException(error);
            } finally {
                bird.discard();
                other.discard();
                level.removePlayerImmediately(player, Entity.RemovalReason.DISCARDED);
            }
        }));
    }
}

package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.alligator.Alligator;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.world.entity.TamableAnimal;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.ai.goal.GoalSelector;
import net.minecraft.world.entity.ai.goal.WrappedGoal;
import net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal;
import net.minecraft.world.level.GameType;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class AmphibiousWetGuestProbe {
    public AmphibiousWetGuestProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("amphibious_wet_guest_fixture")
            .executes(command -> capture(command.getSource(), false)));
        event.getDispatcher().register(Commands.literal("amphibious_shallow_wet_guest_fixture")
            .executes(command -> capture(command.getSource(), true)));
    }

    private static int capture(CommandSourceStack source, boolean shallow) {
        var level = source.getLevel();
        double nearX = shallow ? 6.5 : 8.5;
        double dryY = shallow ? 94 : 96;
        var host = player(level, "DryHost", nearX, dryY, 10.5);
        var guest = player(level, "WetGuest", 8.5, 92, shallow ? 14.25 : 16.5);
        var result = new LinkedHashMap<String, Object>();
        try {
            var contact = Entity.class.getDeclaredMethod("updateFluidInteraction");
            contact.setAccessible(true);
            contact.invoke(host);
            contact.invoke(guest);
            var selectors = Mob.class.getDeclaredField("targetSelector");
            selectors.setAccessible(true);
            for (String species : List.of("hippo", "alligator")) {
                var cases = new LinkedHashMap<String, Object>();
                for (double hostX : List.of(nearX, 23.5)) {
                    host.snapTo(hostX, dryY, 10.5, 0, 0);
                    contact.invoke(host);
                    var kind = species.equals("hippo") ? NaturalistEntityTypes.HIPPO : NaturalistEntityTypes.ALLIGATOR;
                    var mob = kind.get().create(level, EntitySpawnReason.COMMAND);
                    mob.snapTo(8.5, 92, 10.5, 0, 0);
                    level.addFreshEntity(mob);
                    contact.invoke(mob);
                    int priority = species.equals("hippo") ? 4 : 2;
                    var goal = ((GoalSelector) selectors.get(mob)).getAvailableGoals().stream()
                        .filter(entry -> entry.getPriority() == priority).map(WrappedGoal::getGoal)
                        .filter(NearestAttackableTargetGoal.class::isInstance).findFirst().orElseThrow();
                    pickTarget(goal);
                    var outcome = new LinkedHashMap<String, Object>();
                    outcome.put("host_wet", host.isInWater());
                    outcome.put("guest_wet", guest.isInWater());
                    outcome.put("picked_wet_guest", mob.getTarget() == guest);
                    outcome.put("picked_dry_host", mob.getTarget() == host);
                    outcome.put("bright_outside", level.isBrightOutside());
                    if (mob instanceof TamableAnimal pet) {
                        pet.tame(guest);
                        mob.setTarget(null);
                        pickTarget(goal);
                        outcome.put("wet_owner_ignored", mob.getTarget() != guest);
                    } else if (mob instanceof Alligator alligator) {
                        alligator.setHasEgg(true);
                        mob.setTarget(null);
                        pickTarget(goal);
                        outcome.put("picked_visible_dry_host_when_defensive", mob.getTarget() == host);
                        alligator.setHasEgg(false);
                        mob.setTarget(null);
                        var nestGoal = ((GoalSelector) selectors.get(mob)).getAvailableGoals().stream()
                            .filter(entry -> entry.getPriority() == 1).map(WrappedGoal::getGoal)
                            .filter(NearestAttackableTargetGoal.class::isInstance).findFirst().orElseThrow();
                        pickTarget(nestGoal);
                        outcome.put("picked_visible_dry_host_at_nest", mob.getTarget() == host);
                        outcome.put("picked_wet_guest_at_nest", mob.getTarget() == guest);
                    }
                    cases.put(Double.toString(hostX), outcome);
                    mob.discard();
                }
                result.put(species, cases);
            }
        } catch (ReflectiveOperationException failure) {
            throw new IllegalStateException(failure);
        } finally {
            level.removePlayerImmediately(host, Entity.RemovalReason.DISCARDED);
            level.removePlayerImmediately(guest, Entity.RemovalReason.DISCARDED);
        }
        source.sendSuccess(() -> Component.literal("amphibious_wet_guest_fixture " + new Gson().toJson(result)), false);
        return 1;
    }

    private static void pickTarget(net.minecraft.world.entity.ai.goal.Goal goal) {
        for (int attempt = 0; attempt < 100; attempt++) {
            if (goal.canUse()) {
                goal.start();
                return;
            }
        }
    }

    private static ServerPlayer player(ServerLevel level, String name, double x, double y, double z) {
        var profile = new GameProfile(UUID.randomUUID(), name);
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        player.connection = new ServerGamePacketListenerImpl(level.getServer(),
            new Connection(PacketFlow.SERVERBOUND), player, CommonListenerCookie.createInitial(profile, false));
        player.snapTo(x, y, z, 0, 0);
        level.addNewPlayer(player);
        player.connection.markClientLoaded();
        player.setGameMode(GameType.SURVIVAL);
        return player;
    }
}

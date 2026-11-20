package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.item.KnapsackItem;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.UUID;
import net.minecraft.ChatFormatting;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.component.DataComponents;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.resources.Identifier;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.AgeableMob;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.monster.zombie.Zombie;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.TooltipFlag;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.item.component.TooltipDisplay;
import net.minecraft.world.item.context.UseOnContext;
import net.minecraft.world.level.GameType;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class KnapsackProbe {
    public KnapsackProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("knapsack_probe").executes(context -> {
            var result = observe(context.getSource().getLevel());
            context.getSource().sendSuccess(() -> Component.literal("knapsack_probe " + new Gson().toJson(result)), false);
            return 1;
        }));
    }

    private static Mob spawn(ServerLevel level, String kind, boolean baby) {
        var mob = (Mob) BuiltInRegistries.ENTITY_TYPE.getValue(Identifier.parse(kind)).create(level, EntitySpawnReason.COMMAND);
        mob.snapTo(8.5, 102.0, 8.5, 47.0F, 12.0F);
        mob.setNoAi(true);
        if (mob instanceof AgeableMob ageable) {
            ageable.setAge(baby ? -1234 : 0);
        } else if (mob instanceof Zombie zombie) {
            zombie.setBaby(baby);
        }
        level.addFreshEntity(mob);
        return mob;
    }

    private static List<String> tooltip(ServerPlayer player, KnapsackItem sack, ItemStack stack) {
        var lines = new ArrayList<Component>();
        sack.appendHoverText(stack, Item.TooltipContext.of(player.level(), player), TooltipDisplay.DEFAULT, lines::add, TooltipFlag.NORMAL);
        return lines.stream().map(line -> line.getString() + "|" + String.format("%06x", line.getStyle().getColor().getValue())).toList();
    }

    private static LinkedHashMap<String, Object> observe(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "KnapsackFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), new Connection(PacketFlow.SERVERBOUND),
            player, CommonListenerCookie.createInitial(profile, false));
        player.snapTo(8.5, 102.0, 6.5, 0, 0);
        level.addNewPlayer(player);
        player.setGameMode(GameType.SURVIVAL);

        var hand = InteractionHand.MAIN_HAND;
        var sack = NaturalistRegistry.KNAPSACK.get();
        var output = new LinkedHashMap<String, Object>();
        var cow = spawn(level, "minecraft:cow", true);
        var deer = spawn(level, "naturalist:deer", true);
        var zombie = spawn(level, "minecraft:zombie", true);
        var adult = spawn(level, "naturalist:deer", false);
        var vehicle = spawn(level, "minecraft:pig", true);
        var rider = spawn(level, "minecraft:chicken", true);

        try {
            rider.startRiding(vehicle, true, false);
            if (!rider.isPassenger()) {
                throw new IllegalStateException("Fixture rider did not mount");
            }
            output.put("eligible", List.of(KnapsackItem.isCapturable(cow), KnapsackItem.isCapturable(deer),
                KnapsackItem.isCapturable(zombie), KnapsackItem.isCapturable(adult), KnapsackItem.isCapturable(rider),
                KnapsackItem.isCapturable(vehicle)));
            var empty = new ItemStack(sack);
            output.put("default_max_stack", empty.getMaxStackSize());
            output.put("empty_tooltip", tooltip(player, sack, empty));

            deer.setCustomName(Component.literal("Twig").withStyle(ChatFormatting.RED));
            deer.setHealth(5.0F);
            var uuid = deer.getUUID();
            player.setItemInHand(hand, empty);
            var capture = sack.interactLivingEntity(empty, player, deer, hand);
            var full = player.getItemInHand(hand);
            var tag = full.get(DataComponents.CUSTOM_DATA).copyTag();
            output.put("capture", List.of(capture.consumesAction(), deer.isRemoved(), KnapsackItem.isFilled(full),
                tag.getStringOr("id", ""), tag.getIntOr("Age", 0), tag.getFloatOr("Health", 0.0F)));
            output.put("named_tooltip", tooltip(player, sack, full));
            output.put("filled_rejects_second", !sack.interactLivingEntity(full, player, cow, hand).consumesAction());

            var east = new BlockHitResult(new Vec3(10.0, 102.5, 8.5), Direction.EAST, new BlockPos(9, 102, 8), false);
            var released = sack.useOn(new UseOnContext(player, hand, east));
            var found = (Mob) level.getEntity(uuid);
            output.put("release", List.of(released.consumesAction(), !KnapsackItem.isFilled(player.getItemInHand(hand)),
                found.getX(), found.getY(), found.getZ(), ((AgeableMob) found).getAge(), found.getHealth(),
                found.getCustomName().getString(), found.getYRot(), found.getXRot(), found.isPersistenceRequired()));
            found.discard();

            var off = InteractionHand.OFF_HAND;
            var otherEmpty = new ItemStack(sack);
            player.setItemInHand(off, otherEmpty);
            sack.interactLivingEntity(otherEmpty, player, zombie, off);
            var capturedZombie = player.getItemInHand(off);
            output.put("unnamed_tooltip", tooltip(player, sack, capturedZombie));
            var zombieUuid = zombie.getUUID();
            sack.useOn(new UseOnContext(player, off, east));
            var reloadedZombie = (Zombie) level.getEntity(zombieUuid);
            output.put("offhand_non_ageable_baby", List.of(reloadedZombie.isBaby(),
                !KnapsackItem.isFilled(player.getItemInHand(off))));
            reloadedZombie.discard();

            player.setItemInHand(hand, new ItemStack(sack));
            var vehicleUuid = vehicle.getUUID();
            var riderUuid = rider.getUUID();
            var captureVehicle = sack.interactLivingEntity(player.getItemInHand(hand), player, vehicle, hand);
            var vehicleTag = player.getItemInHand(hand).get(DataComponents.CUSTOM_DATA).copyTag();
            var beforeReleaseRider = level.getEntity(riderUuid);
            boolean originalDetached = beforeReleaseRider == rider && !rider.isPassenger();
            sack.useOn(new UseOnContext(player, hand, east));
            var restoredVehicle = (Mob) level.getEntity(vehicleUuid);
            output.put("vehicle", List.of(captureVehicle.consumesAction(), vehicleTag.contains("Passengers"),
                originalDetached, restoredVehicle.getPassengers().size(), level.getEntity(riderUuid) == rider,
                rider.isPassenger()));
            restoredVehicle.discard();

            player.setItemInHand(hand, new ItemStack(sack, 2));
            sack.interactLivingEntity(player.getItemInHand(hand), player, cow, hand);
            var kept = player.getItemInHand(hand);
            int filledOther = 0;
            for (int i = 1; i < 36; i++) {
                var item = player.getInventory().getItem(i);
                if (item.is(sack) && KnapsackItem.isFilled(item)) {
                    filledOther += item.getCount();
                }
            }
            output.put("overstack_capture", List.of(kept.getCount(), !KnapsackItem.isFilled(kept), filledOther));

            player.getInventory().clearContent();
            var overfilled = new ItemStack(sack, 2);
            var overfilledData = new CompoundTag();
            overfilledData.putString("id", "minecraft:cow");
            overfilledData.putInt("Age", -1234);
            overfilled.set(DataComponents.CUSTOM_DATA, CustomData.of(overfilledData));
            player.setItemInHand(hand, overfilled);
            var overRelease = sack.useOn(new UseOnContext(player, hand, east));
            int emptyOther = 0;
            for (int i = 1; i < 36; i++) {
                var item = player.getInventory().getItem(i);
                if (item.is(sack) && !KnapsackItem.isFilled(item)) {
                    emptyOther += item.getCount();
                }
            }
            output.put("overstack_release", List.of(overRelease.consumesAction(),
                player.getItemInHand(hand).getCount(), KnapsackItem.isFilled(player.getItemInHand(hand)), emptyOther));

            var bad = new ItemStack(sack);
            var invalid = new CompoundTag();
            invalid.putString("id", "naturalist:does_not_exist");
            bad.set(DataComponents.CUSTOM_DATA, CustomData.of(invalid));
            player.setItemInHand(hand, bad);
            output.put("invalid_release_clears", List.of(sack.useOn(new UseOnContext(player, hand, east)).consumesAction(),
                !KnapsackItem.isFilled(player.getItemInHand(hand))));
            return output;
        } finally {
            for (LivingEntity entity : List.of(cow, deer, zombie, adult, vehicle, rider)) {
                entity.discard();
            }
            player.discard();
        }
    }
}

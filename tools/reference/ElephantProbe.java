package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.inventory.ElephantInventoryMenu;
import com.crispytwig.naturalist.world.entity.animal.NaturalistAnimal;
import com.crispytwig.naturalist.world.entity.animal.elephant.Elephant;
import com.crispytwig.naturalist.world.entity.animal.elephant.Mammoth;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.util.ProblemReporter;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.inventory.ContainerInput;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.storage.TagValueOutput;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class ElephantProbe {
    public ElephantProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_elephant_fixture").executes(context -> {
            var source = context.getSource();
            var level = source.getLevel();
            var owner = player(level);
            var records = new LinkedHashMap<String, Object>();
            try {
                for (String species : List.of("elephant", "mammoth")) {
                    var animal = create(level, species);
                    var record = new LinkedHashMap<String, Object>();
                    record.put("variant", animal.getVariantString());
                    record.put("dimensions", List.of(animal.getBbWidth(), animal.getBbHeight(), animal.getEyeHeight()));
                    record.put("attributes", List.of(animal.getMaxHealth(),
                        animal.getAttributeBaseValue(Attributes.MOVEMENT_SPEED),
                        animal.getAttributeBaseValue(Attributes.ATTACK_DAMAGE),
                        animal.getAttributeBaseValue(Attributes.ATTACK_KNOCKBACK),
                        animal.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE),
                        animal.getAttributeBaseValue(Attributes.FOLLOW_RANGE),
                        animal.getAttributeBaseValue(Attributes.STEP_HEIGHT)));
                    record.put("head_turn", animal.getMaxHeadYRot());
                    record.put("melon_is_food", animal.isFood(new ItemStack(Items.MELON_SLICE)));
                    record.put("cake_is_food", animal.isFood(new ItemStack(Items.CAKE)));
                    animal.setAge(-24000);
                    record.put("juvenile_dimensions", List.of(animal.getBbWidth(), animal.getBbHeight(), animal.getEyeHeight()));
                    List<Boolean> taming = new ArrayList<>();
                    ItemStack cake = new ItemStack(Items.CAKE, 5);
                    owner.setItemInHand(InteractionHand.MAIN_HAND, cake);
                    for (int i = 0; i < 5; i++) {
                        animal.mobInteract(owner, InteractionHand.MAIN_HAND);
                        taming.add(animal.isTame() && animal.isOwnedBy(owner));
                    }
                    record.put("baby_taming", taming);
                    record.put("cake_remaining", cake.getCount());
                    record.put("age_after_cake", animal.getAge());
                    var saved = TagValueOutput.createWithContext(ProblemReporter.DISCARDING, level.registryAccess());
                    animal.addAdditionalSaveData(saved);
                    record.put("taming_count_saved", saved.buildResult().getIntOr("TamingFood", -1));
                    animal.setAge(0);
                    animal.setHealth(52);
                    var melon = new ItemStack(Items.MELON_SLICE, 2);
                    owner.setItemInHand(InteractionHand.MAIN_HAND, melon);
                    animal.mobInteract(owner, InteractionHand.MAIN_HAND);
                    record.put("healed_to", animal.getHealth());
                    record.put("melon_remaining", melon.getCount());
                    for (var item : List.of(Items.CHEST, Items.SADDLE)) {
                        owner.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(item, 1));
                        animal.mobInteract(owner, InteractionHand.MAIN_HAND);
                    }
                    record.put("chested", animal.isChested());
                    record.put("saddled", animal.isSaddled());
                    owner.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
                    animal.mobInteract(owner, InteractionHand.MAIN_HAND);
                    record.put("owner_mounted", owner.getVehicle() == animal);
                    var seat = animal.getPassengerRidingPosition(owner).subtract(animal.position());
                    record.put("seat", List.of(seat.x, seat.y, seat.z));
                    record.put("pushable_mounted", animal.isPushable());
                    owner.stopRiding();
                    record.put("custom_inventory", inventory(animal, owner));
                    var shears = new ItemStack(Items.SHEARS);
                    owner.setItemInHand(InteractionHand.MAIN_HAND, shears);
                    animal.mobInteract(owner, InteractionHand.MAIN_HAND);
                    record.put("shear_chest_first", !animal.isChested() && animal.isSaddled());
                    animal.mobInteract(owner, InteractionHand.MAIN_HAND);
                    record.put("shear_saddle_second", !animal.isSaddled());
                    record.put("shear_damage", shears.getDamageValue());
                    var zombie = EntityTypes.ZOMBIE.create(level, EntitySpawnReason.COMMAND);
                    zombie.snapTo(9.5, 92, 8.5, 0, 0);
                    zombie.setNoAi(true);
                    zombie.setOnGround(true);
                    level.addFreshEntity(zombie);
                    record.put("real_attack_hits", animal.doHurtTarget(level, zombie));
                    record.put("target_health", zombie.getHealth());
                    var push = zombie.getDeltaMovement();
                    record.put("target_motion", List.of(push.x, push.y, push.z));
                    record.put("can_attack_owner", animal.canAttack(owner));
                    zombie.discard();

                    var knockback = new ArrayList<Double>();
                    for (int age : new int[]{0, -24000}) {
                        animal.setAge(age);
                        animal.setOnGround(true);
                        animal.setDeltaMovement(Vec3.ZERO);
                        animal.knockback(1.0, 1.0, 0.0, level.damageSources().generic(), 0.0f, false);
                        knockback.add(animal.getDeltaMovement().x);
                    }
                    record.put("adult_and_baby_knockback", knockback);
                    level.getRandom().setSeed(641127L);
                    record.put("seeded_natural_group", group(level, species));
                    record.put("spawn_substrate", substrates(level, species));
                    record.put("live_revenge", retaliation(level, species));
                    records.put(species, record);
                    animal.discard();
                }
                source.sendSuccess(() -> Component.literal("naturalist_elephant_fixture " + new Gson().toJson(records)), false);
            } finally {
                owner.discard();
            }
            return 1;
        }));
    }

    private static Map<String, Object> retaliation(ServerLevel level, String species) {
        int y = 150;
        var stone = net.minecraft.core.registries.BuiltInRegistries.BLOCK.getValue(
            net.minecraft.resources.Identifier.withDefaultNamespace("stone")).defaultBlockState();
        for (int x = 2; x < 15; x++) {
            for (int z = 2; z < 15; z++) {
                level.setBlockAndUpdate(new BlockPos(x, y - 1, z), stone);
            }
        }
        var animal = create(level, species);
        animal.snapTo(8.5, y, 8.5, 0, 0);
        animal.setNoAi(false);
        animal.setOnGround(true);
        var zombie = EntityTypes.ZOMBIE.create(level, EntitySpawnReason.COMMAND);
        zombie.snapTo(9.5, y, 8.5, 0, 0);
        zombie.setNoAi(true);
        zombie.setOnGround(true);
        level.addFreshEntity(zombie);
        var result = new LinkedHashMap<String, Object>();
        result.put("mob_hit", animal.hurtServer(level, animal.damageSources().mobAttack(zombie), 1.0F));
        result.put("initial_health", animal.getHealth());
        int targetTick = -1;
        int attackTick = -1;
        for (int tick = 1; tick <= 60; tick++) {
            level.tickNonPassenger(animal);
            if (animal.getTarget() == zombie && targetTick < 0) {
                targetTick = tick;
            }
            if (zombie.getHealth() < 20) {
                attackTick = tick;
                break;
            }
        }
        result.put("first_target_tick", targetTick);
        result.put("first_attack_tick", attackTick);
        result.put("target_health", zombie.getHealth());
        result.put("distance_at_end", animal.distanceTo(zombie));
        animal.discard();
        zombie.discard();
        return result;
    }

    private static Map<String, Boolean> substrates(ServerLevel level, String species) {
        var result = new LinkedHashMap<String, Boolean>();
        var position = new BlockPos(12, 130, 12);
        for (String id : List.of("grass_block", "sand", "stone", "snow_block", "ice", "packed_ice")) {
            var key = net.minecraft.resources.Identifier.withDefaultNamespace(id);
            var block = net.minecraft.core.registries.BuiltInRegistries.BLOCK.getValue(key);
            level.setBlockAndUpdate(position.below(), block.defaultBlockState());
            var random = net.minecraft.util.RandomSource.create(3781L);
            boolean allowed = species.equals("elephant")
                ? NaturalistAnimal.checkNaturalistAnimalSpawnRules(NaturalistEntityTypes.ELEPHANT.get(),
                    level, EntitySpawnReason.NATURAL, position, random)
                : Mammoth.checkMammothSpawnRules(NaturalistEntityTypes.MAMMOTH.get(),
                    level, EntitySpawnReason.NATURAL, position, random);
            result.put(id, allowed);
        }
        return result;
    }

    private static Map<String, Object> inventory(Elephant animal, ServerPlayer owner) {
        var record = new LinkedHashMap<String, Object>();
        owner.setShiftKeyDown(true);
        owner.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
        animal.mobInteract(owner, InteractionHand.MAIN_HAND);
        owner.setShiftKeyDown(false);
        if (!(owner.containerMenu instanceof ElephantInventoryMenu menu)) {
            record.put("opened", false);
            return record;
        }
        record.put("opened", true);
        record.put("entity", menu.getElephant() == animal);
        record.put("slot_count", menu.slots.size());
        record.put("chest_slot_active", menu.getSlot(24).isActive());
        record.put("chest_first_xy", List.of(menu.getSlot(0).x, menu.getSlot(0).y));
        record.put("chest_last_xy", List.of(menu.getSlot(24).x, menu.getSlot(24).y));
        record.put("saddle_xy", List.of(menu.getSlot(25).x, menu.getSlot(25).y));
        record.put("banner_xy", List.of(menu.getSlot(26).x, menu.getSlot(26).y));
        record.put("saddle_accepts_dirt", menu.getSlot(25).mayPlace(new ItemStack(Items.DIRT)));
        record.put("banner_accepts_dirt", menu.getSlot(26).mayPlace(new ItemStack(Items.DIRT)));
        record.put("saddle_limit", menu.getSlot(25).getMaxStackSize());
        record.put("banner_limit", menu.getSlot(26).getMaxStackSize());
        owner.getInventory().setItem(9, new ItemStack(Items.STONE, 19));
        owner.getInventory().setItem(10, new ItemStack(net.minecraft.core.registries.BuiltInRegistries.ITEM.getValue(net.minecraft.resources.Identifier.withDefaultNamespace("red_banner")), 2));
        record.put("moved_stone", menu.quickMoveStack(owner, 27).getCount());
        record.put("stone_in_first", menu.getSlot(0).getItem().getCount());
        record.put("moved_banner", menu.quickMoveStack(owner, 28).getCount());
        record.put("banner_in_slot", menu.getSlot(26).getItem().getCount());
        record.put("banner_left_in_player", owner.getInventory().getItem(10).getCount());
        var banner = menu.getSlot(26).getItem().copy();
        menu.getSlot(26).set(ItemStack.EMPTY);
        owner.getInventory().setItem(10, banner.copyWithCount(2));
        menu.clicked(28, 0, ContainerInput.QUICK_MOVE, owner);
        record.put("shift_banner_in_equipment", menu.getSlot(26).getItem().getCount());
        record.put("shift_banner_in_chest", menu.getSlot(1).getItem().getCount());
        record.put("shift_banner_left_in_player", owner.getInventory().getItem(10).getCount());

        record.put("owner_can_use", menu.stillValid(owner));
        owner.snapTo(16.5, 92, 8.5, 0, 0);
        record.put("owner_exactly_eight_can_use", menu.stillValid(owner));
        owner.snapTo(8.5, 92, 7.5, 0, 0);
        owner.closeContainer();
        return record;
    }

    private static List<Map<String, Object>> group(ServerLevel level, String species) {
        var members = new ArrayList<Map<String, Object>>();
        net.minecraft.world.entity.SpawnGroupData group = null;
        for (int index = 0; index < 5; index++) {
            var member = create(level, species);
            group = member.finalizeSpawn(level, level.getCurrentDifficultyAt(new BlockPos(8, 92, 8)),
                EntitySpawnReason.NATURAL, group);
            members.add(Map.of("baby", member.isBaby(),
                "follow_range", member.getAttributeValue(Attributes.FOLLOW_RANGE)));
            member.discard();
        }
        return members;
    }

    private static Elephant create(ServerLevel level, String species) {
        Elephant mob = species.equals("elephant")
            ? NaturalistEntityTypes.ELEPHANT.get().create(level, EntitySpawnReason.COMMAND)
            : NaturalistEntityTypes.MAMMOTH.get().create(level, EntitySpawnReason.COMMAND);
        mob.snapTo(8.5, 92, 8.5, 0, 0);
        mob.setNoAi(true);
        mob.setNoGravity(true);
        level.addFreshEntity(mob);
        return mob;
    }

    private static ServerPlayer player(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "ElephantFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(8.5, 92, 7.5, 0, 0);
        level.addNewPlayer(player);
        player.setGameMode(GameType.SURVIVAL);
        return player;
    }
}

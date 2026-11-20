package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.ai.goal.SleepGoal;
import com.crispytwig.naturalist.world.entity.animal.boar.Boar;
import com.crispytwig.naturalist.world.entity.variant.DataDrivenVariantAnimal;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.damagesource.DamageSource;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.NeutralMob;
import net.minecraft.world.entity.ai.goal.GoalSelector;
import net.minecraft.world.entity.ai.goal.target.HurtByTargetGoal;
import net.minecraft.world.entity.ai.goal.target.ResetUniversalAngerTargetGoal;
import net.minecraft.world.level.gamerules.GameRules;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.animal.Animal;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

/** Records pinned Java animal attributes, dimensions, food and interaction behavior. */
@Mod("naturalist_fixture")
public final class LandProbe {
    public LandProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_land")
                .executes(ctx -> {
                    try { return capture(ctx.getSource()); }
                    catch (RuntimeException failure) { failure.printStackTrace(); throw failure; }
                }));
    }

    private static int capture(CommandSourceStack source) {
        ServerLevel level = source.getLevel();
        var result = new LinkedHashMap<String, Object>();
        result.put("boar", animal(level, NaturalistEntityTypes.BOAR.get()));
        result.put("turkey", animal(level, NaturalistEntityTypes.TURKEY.get()));
        result.put("capybara", capybara(level));
        result.put("capybara_owner", capybaraOwner(level));
        var boar = create(level, NaturalistEntityTypes.BOAR.get());
        boar.setHealth(12.0F);
        boolean aggroAtHalf = boar.isAggro();
        boar.setHealth(12.5F);
        boolean aggroAboveHalf = boar.isAggro();
        boar.setAge(-24000);
        var bolt = EntityTypes.LIGHTNING_BOLT.create(level, EntitySpawnReason.COMMAND);
        boar.snapTo(9.5D, 71D, 7.5D, 75F, 0F);
        boar.setNoAi(true);
        boar.setCustomName(Component.literal("Fixture Boar"));
        boar.setCustomNameVisible(true);
        boar.thunderHit(level, bolt);
        var zoglin = level.getEntitiesOfClass(net.minecraft.world.entity.monster.Zoglin.class,
                boar.getBoundingBox().inflate(2.0D)).stream().findFirst().orElseThrow();
        result.put("boar_behavior", Map.of("aggro_at_half", aggroAtHalf,
                "aggro_above_half", aggroAboveHalf, "lightning_baby", zoglin.isBaby(),
                "lightning_no_ai", zoglin.isNoAi(), "lightning_name_visible", zoglin.isCustomNameVisible(),
                "lightning_removed", boar.isRemoved(), "lightning_name", zoglin.getName().getString()));
        zoglin.discard();
        result.put("boar_disc", boarDisc(level));
        result.put("universal_anger", Map.of(
                "boar", universalAnger(level, NaturalistEntityTypes.BOAR.get()),
                "bear", universalAnger(level, NaturalistEntityTypes.BEAR.get()),
                "black_bear", universalAnger(level, NaturalistEntityTypes.BLACK_BEAR.get())));
        source.sendSuccess(() -> Component.literal("naturalist_land " + new Gson().toJson(result)), false);
        return 1;
    }

    private static <T extends Mob & NeutralMob> Map<String, Map<String, Boolean>> universalAnger(
            ServerLevel level, EntityType<T> kind) {
        var rules = level.getGameRules();
        var enabledBefore = rules.get(GameRules.UNIVERSAL_ANGER);
        var result = new LinkedHashMap<String, Map<String, Boolean>>();
        var attacker = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "UniversalAttacker"));
        var other = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "UniversalBystander"));
        attacker.setPermanentlyInvulnerable(false);
        other.setPermanentlyInvulnerable(false);
        try {
            var selector = Mob.class.getDeclaredField("targetSelector");
            selector.setAccessible(true);
            for (boolean enabled : new boolean[] {true, false}) {
                rules.set(GameRules.UNIVERSAL_ANGER, enabled, level.getServer());
                var mob = kind.create(level, EntitySpawnReason.COMMAND);
                mob.tickCount = 20;
                mob.snapTo(8.5, 71.0, 8.5, 0, 0);
                attacker.snapTo(8.5, 71.0, 9.5, 0, 0);
                other.snapTo(8.5, 71.0, 7.5, 0, 0);
                mob.setLastHurtByMob(attacker);
                var goals = ((GoalSelector) selector.get(mob)).getAvailableGoals();
                if (enabled) {
                    var reset = goals.stream().filter(goal -> goal.getGoal() instanceof ResetUniversalAngerTargetGoal<?>)
                            .findFirst().orElseThrow();
                    if (!reset.canUse()) {
                        throw new IllegalStateException("Universal anger could not start after a player hit");
                    }
                    reset.start();
                } else {
                    var retaliate = goals.stream().filter(goal -> goal.getGoal() instanceof HurtByTargetGoal
                            && goal.canUse()).findFirst().orElseThrow();
                    retaliate.start();
                    mob.updatePersistentAnger(level, true);
                }
                result.put(Boolean.toString(enabled), Map.of("retaliated", mob.isAngryAt(attacker, level),
                        "includes_bystander", mob.isAngryAt(other, level)));
            }
        } catch (ReflectiveOperationException failure) {
            throw new IllegalStateException(failure);
        } finally {
            rules.set(GameRules.UNIVERSAL_ANGER, enabledBefore, level.getServer());
        }
        return result;
    }

    private static Map<String, Boolean> capybaraOwner(ServerLevel level) {
        var owner = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "CapybaraOwner"));
        var guest = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "CapybaraGuest"));
        var animal = create(level, NaturalistEntityTypes.CAPYBARA.get());
        var hand = InteractionHand.MAIN_HAND;
        var sample = new LinkedHashMap<String, Boolean>();
        animal.tame(owner);
        animal.setHealth(4.0F);
        animal.setFollowingOwner(true);
        guest.setItemInHand(hand, ItemStack.EMPTY);
        animal.mobInteract(guest, hand);
        sample.put("guest_toggles_sit", animal.isOrderedToSit());
        guest.setItemInHand(hand, new ItemStack(NaturalistRegistry.WHISTLE.get()));
        animal.mobInteract(guest, hand);
        sample.put("guest_toggles_follow", !animal.isFollowingOwner());
        guest.setItemInHand(hand, new ItemStack(Items.MELON_SLICE, 2));
        animal.mobInteract(guest, hand);
        sample.put("guest_heals", animal.getHealth() > 4.0F);
        sample.put("guest_breeds", animal.isInLove());
        sample.put("guest_food_consumed", guest.getItemInHand(hand).getCount() == 1);
        owner.setItemInHand(hand, new ItemStack(Items.MELON_SLICE, 2));
        animal.mobInteract(owner, hand);
        sample.put("owner_heals", animal.getHealth() == 6.0F);
        owner.setItemInHand(hand, ItemStack.EMPTY);
        animal.mobInteract(owner, hand);
        sample.put("owner_toggles_sit", animal.isOrderedToSit());
        return sample;
    }

    private static <T extends Animal & DataDrivenVariantAnimal> Map<String, Object> animal(
            ServerLevel level, EntityType<T> kind) {
        T adult = create(level, kind);
        T partner = create(level, kind);
        T juvenile = create(level, kind);
        juvenile.setAge(-24000);
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "LandFixture"));
        final var hand = InteractionHand.MAIN_HAND;
        var result = new LinkedHashMap<String, Object>();
        result.put("max_health", adult.getAttributeBaseValue(Attributes.MAX_HEALTH));
        result.put("movement_speed", adult.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
        result.put("attack_damage", adult.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
        if (adult.getAttribute(Attributes.ATTACK_KNOCKBACK) != null) {
            result.put("attack_knockback", adult.getAttributeBaseValue(Attributes.ATTACK_KNOCKBACK));
        }
        result.put("knockback_resistance", adult.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE));
        result.put("follow_range", adult.getAttributeBaseValue(Attributes.FOLLOW_RANGE));
        result.put("step_height", adult.getAttributeBaseValue(Attributes.STEP_HEIGHT));
        result.put("variant", adult.getVariantString());
        result.put("adult", dimensions(adult));
        result.put("juvenile", dimensions(juvenile));
        var inputs = Map.of("carrot", Items.CARROT, "potato", Items.POTATO, "beetroot", Items.BEETROOT,
                "wheat_seeds", Items.WHEAT_SEEDS, "beetroot_seeds", Items.BEETROOT_SEEDS,
                "melon_seeds", Items.MELON_SEEDS, "pumpkin_seeds", Items.PUMPKIN_SEEDS,
                "pitcher_pod", Items.PITCHER_POD, "torchflower_seeds", Items.TORCHFLOWER_SEEDS, "wheat", Items.WHEAT);
        result.put("food", inputs.entrySet().stream().filter(entry -> adult.isFood(new ItemStack(entry.getValue())))
                .map(Map.Entry::getKey).sorted().toList());
        var breedingFood = kind == NaturalistEntityTypes.BOAR.get() ? Items.CARROT : Items.WHEAT_SEEDS;
        player.setItemInHand(hand, new ItemStack(breedingFood, 2));
        adult.mobInteract(player, hand);
        result.put("fed_adult", adult.isInLove());
        result.put("remaining_feed", player.getItemInHand(hand).getCount());
        partner.setInLove(player);
        result.put("same_species_mate", adult.canMate(partner));
        adult.setVariantString("fixture:from_first_parent");
        partner.setVariantString("fixture:from_second_parent");
        result.put("offspring_variants", java.util.stream.IntStream.range(0, 16)
                .mapToObj(index -> {
                    adult.getRandom().setSeed(index * 4096L);
                    return ((DataDrivenVariantAnimal)adult.getBreedOffspring(level, partner)).getVariantString();
                }).distinct().sorted().toList());
        var damage = juvenile.damageSources().fall();
        float beforeFall = juvenile.getHealth();
        juvenile.causeFallDamage(30.0D, 1.0F, damage);
        result.put("fall_damage", beforeFall > juvenile.getHealth());
        return result;
    }

    private static Map<String, Object> boarDisc(ServerLevel level) {
        var boar = create(level, NaturalistEntityTypes.BOAR.get());
        boar.snapTo(6.5D, 71D, 9.5D, 0, 0);
        var item = NaturalistRegistry.MUSIC_DISC_DEATH_BY_HOGS.get();
        var results = new LinkedHashMap<String, Object>();
        int hits = 0;
        int fatal = 0;
        int ordinary = 0;
        int discs = 0;
        for (int i = 0; i < 96; i++) {
            var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "BoarDiscFixture")) {
                @Override
                public boolean hurtServer(ServerLevel world, DamageSource source, float damage) {
                    setHealth(getHealth() - damage);
                    return damage > 0;
                }
            };
            player.snapTo(6.5D, 71D, 8.5D, 0, 0);
            player.setHealth(i == 0 ? 10F : 0.5F);
            boar.getRandom().setSeed(0x5DEECE66DL + 1442695040888963407L * i);
            if (boar.doHurtTarget(level, player)) {
                hits++;
                if (player.isDeadOrDying()) {
                    fatal++;
                } else {
                    ordinary++;
                }
            }
            var found = level.getEntitiesOfClass(ItemEntity.class, player.getBoundingBox().inflate(2),
                    e -> e.getItem().is(item));
            discs += found.size();
            found.forEach(ItemEntity::discard);
            player.discard();
        }
        results.put("landed_hits", hits);
        results.put("fatal_hits", fatal);
        results.put("nonfatal_hits", ordinary);
        results.put("discs", discs);
        boar.discard();
        return results;
    }

    private static Map<String, Object> capybara(ServerLevel level) {
        var kind = NaturalistEntityTypes.CAPYBARA.get();
        var adult = create(level, kind);
        var partner = create(level, kind);
        var young = create(level, kind);
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "CapybaraFixture"));
        var result = new LinkedHashMap<String, Object>();
        var hand = InteractionHand.MAIN_HAND;
        young.setAge(-24000);
        result.put("max_health", adult.getAttributeBaseValue(Attributes.MAX_HEALTH));
        result.put("movement_speed", adult.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
        result.put("adult", dimensions(adult));
        result.put("juvenile", dimensions(young));
        result.put("variant", adult.getVariantString());
        var food = new ItemStack(Items.MELON_SLICE, 2);
        result.put("eats_melon", adult.isFood(food));
        result.put("untamed_wont_mate", !adult.canMate(partner));
        var capybara = create(level, kind);
        player.setItemInHand(hand, food.copy());
        capybara.getRandom().setSeed(1L);
        capybara.mobInteract(player, hand); // nextInt(3) is zero here.
        result.put("tame_failed", !capybara.isTame());
        result.put("failure_consumed", player.getItemInHand(hand).getCount() == 1);
        capybara.getRandom().setSeed(2L);
        capybara.mobInteract(player, hand);
        result.put("tame_succeeded", capybara.isTame());
        result.put("default_dye", capybara.getDyeColor().getName());
        capybara.mobInteract(player, hand); // Empty hand, from its owner.
        result.put("ordered_sit", capybara.isOrderedToSit());
        player.setItemInHand(hand, new ItemStack(NaturalistRegistry.WHISTLE.get()));
        capybara.mobInteract(player, hand);
        result.put("whistle_flipped_following", !capybara.isFollowingOwner());
        result.put("whistle_stood_pet_up", !capybara.isOrderedToSit());
        capybara.setHealth(4F);
        player.setItemInHand(hand, food.copy());
        capybara.mobInteract(player, hand);
        result.put("healed_health", capybara.getHealth());
        result.put("heal_consumed", player.getItemInHand(hand).getCount() == 1);
        adult.tame(player);
        partner.tame(player);
        adult.setInLove(player);
        partner.setInLove(player);
        var offspring = adult.getBreedOffspring(level, partner);
        result.put("tamed_can_mate", adult.canMate(partner));
        result.put("offspring_tamed", offspring.isTame());
        result.put("offspring_owner", offspring.isOwnedBy(player));
        result.put("offspring_dye", offspring.getDyeColor().getName());
        result.put("fall_damage", young.causeFallDamage(30D, 1F, young.damageSources().fall()));
        var nap = new SleepGoal<>(adult);
        adult.resetLove();
        adult.setOnGround(true);
        result.put("can_sleep_at_fixture_time", nap.canUse());
        nap.start();
        result.put("sleep_started", adult.isSleeping());
        nap.stop();
        result.put("sleep_stopped", !adult.isSleeping());
        return result;
    }

    private static List<Float> dimensions(Animal animal) {
        var size = animal.getDimensions(Pose.STANDING);
        return List.of(size.width(), size.height(), size.eyeHeight());
    }

    private static <T extends Animal> T create(ServerLevel level, EntityType<T> kind) {
        var animal = kind.create(level, EntitySpawnReason.COMMAND);
        if (animal == null) {
            throw new IllegalStateException("Missing reference animal");
        }
        return animal;
    }
}

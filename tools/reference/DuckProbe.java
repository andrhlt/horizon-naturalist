package naturalist.fixture;

import java.util.UUID;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.mojang.authlib.GameProfile;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

/** Records original Naturalist interactions in a server without a client. */
@Mod("naturalist_fixture")
public final class DuckProbe {
    public DuckProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("duck_fixture_pets")
                .executes(ctx -> capture(ctx.getSource())));
    }

    private static int capture(CommandSourceStack source) {
        var level = source.getLevel();
        var duck = NaturalistEntityTypes.DUCK.get().create(level, EntitySpawnReason.COMMAND);
        if (duck == null) {
            throw new IllegalStateException("Duck not registered");
        }
        var adult = duck.getDimensions(Pose.STANDING);
        duck.setAge(-12345);
        var baby = duck.getDimensions(Pose.STANDING);
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "DuckFixtureOwner"));
        final var hand = InteractionHand.MAIN_HAND;
        player.setItemInHand(hand, new ItemStack(Items.COD, 8));
        duck.getRandom().setSeed(0);
        duck.mobInteract(player, hand);
        boolean tamed = duck.isTame() && duck.isOwnedBy(player);
        int fish = player.getItemInHand(hand).getCount();
        player.setItemInHand(hand, ItemStack.EMPTY);
        duck.mobInteract(player, hand);
        boolean sat = duck.isOrderedToSit();
        player.setItemInHand(hand, new ItemStack(Items.DYE.red(), 2));
        duck.mobInteract(player, hand);
        int dye = duck.getDyeColor() == null ? -1 : duck.getDyeColor().getId();
        int dyeLeft = player.getItemInHand(hand).getCount();
        player.setItemInHand(hand, new ItemStack(Items.SHEARS));
        duck.mobInteract(player, hand);
        boolean dyeCleared = duck.getDyeColor() == null;
        int shearsWear = player.getItemInHand(hand).getDamageValue();
        player.setItemInHand(hand, new ItemStack(NaturalistRegistry.WHISTLE.get()));
        duck.mobInteract(player, hand);
        boolean following = duck.isFollowingOwner();
        boolean stand = !duck.isOrderedToSit();
        duck.setHealth(1);
        player.setItemInHand(hand, new ItemStack(Items.SALMON, 2));
        duck.mobInteract(player, hand);
        float healed = duck.getHealth();
        player.setItemInHand(hand, new ItemStack(Items.BUCKET));
        duck.mobInteract(player, hand);
        var filled = player.getItemInHand(hand);
        var data = filled.get(DataComponents.BUCKET_ENTITY_DATA);
        var tag = data == null ? new net.minecraft.nbt.CompoundTag() : data.copyTag();
        int age = tag.getIntOr("Age", 0);
        String variant = tag.getStringOr("Variant", "");
        boolean filledDuckBucket = filled.is(NaturalistRegistry.DUCK_BUCKET.get());
        var result = """
                {"adult":[%s,%s,%s],"baby":[%s,%s,%s],"tamed":%s,"fish_left":%d,"sit":%s,
                "dye":%d,"dye_left":%d,"dye_cleared":%s,"shears_wear":%d,"following":%s,
                "stand":%s,"healed":%s,"bucket":%s,"bucket_age":%d,"bucket_variant":"%s"}
                """.formatted(adult.width(), adult.height(), adult.eyeHeight(), baby.width(),
                baby.height(), baby.eyeHeight(), tamed, fish, sat, dye, dyeLeft, dyeCleared,
                shearsWear, following, stand, healed, filledDuckBucket, age, variant).replace("\n", "");
        source.sendSuccess(() -> Component.literal("duck_fixture_pets " + result), false);
        return 1;
    }
}

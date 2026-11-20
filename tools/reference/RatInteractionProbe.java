package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.UUID;
import net.minecraft.network.Connection;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.level.GameType;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class RatInteractionProbe {
    public RatInteractionProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("rat_fixture")
            .then(Commands.literal("setup").executes(ctx -> run(ctx.getSource()))));
    }

    private int run(net.minecraft.commands.CommandSourceStack source) {
        try {
            Map<String, Object> sample = setup(source.getLevel());
            source.sendSuccess(() -> Component.literal("rat_fixture " + new Gson().toJson(sample)), false);
        } catch (Exception ex) {
            ex.printStackTrace();
            source.sendSuccess(() -> Component.literal("rat_fixture_error " + ex), false);
        }
        return 1;
    }

    private static Map<String, Object> setup(ServerLevel level) throws Exception {
        var out = new LinkedHashMap<String, Object>();
        var rat = NaturalistEntityTypes.RAT.get().create(level, EntitySpawnReason.COMMAND);
        rat.snapTo(6.5, 92, 5.5, 0, 0);
        rat.setNoAi(true);
        level.addFreshEntity(rat);
        var profile = new GameProfile(UUID.randomUUID(), "RatFixture");
        var owner = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        owner.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, owner,
                CommonListenerCookie.createInitial(profile, false));
        owner.snapTo(6.5, 92, 7.5, 0, 0);
        level.addNewPlayer(owner);
        owner.setGameMode(GameType.SURVIVAL);
        rat.tame(owner);
        rat.setHealth(rat.getMaxHealth());
        rat.setAge(6000);
        var whistle = new ItemStack(NaturalistRegistry.WHISTLE.get());
        var bread = new ItemStack(Items.BREAD);
        owner.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
        owner.setItemInHand(InteractionHand.OFF_HAND, whistle);
        rat.setOrderedToSit(false);
        rat.setFollowingOwner(true);
        var emptyMain = rat.mobInteract(owner, InteractionHand.MAIN_HAND);
        out.put("empty_main", List.of(emptyMain.consumesAction(), rat.isOrderedToSit(), rat.isFollowingOwner()));

        rat.setOrderedToSit(true);
        rat.setFollowingOwner(true);
        owner.setItemInHand(InteractionHand.MAIN_HAND, bread);
        var breadMain = rat.mobInteract(owner, InteractionHand.MAIN_HAND);
        var offWhistle = rat.mobInteract(owner, InteractionHand.OFF_HAND);
        out.put("bread_main_whistle_off", List.of(breadMain.consumesAction(), offWhistle.consumesAction(),
                rat.isOrderedToSit(), rat.isFollowingOwner(), bread.getCount()));

        owner.setItemInHand(InteractionHand.MAIN_HAND, whistle);
        var repeat = rat.mobInteract(owner, InteractionHand.MAIN_HAND);
        out.put("whistle_during_cooldown", List.of(repeat.consumesAction(), rat.isOrderedToSit(), rat.isFollowingOwner()));
        owner.discard();
        rat.discard();
        return out;
    }

}

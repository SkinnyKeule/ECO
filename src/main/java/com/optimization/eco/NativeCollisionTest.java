package com.optimization.eco;

import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.level.Level;
import net.neoforged.bus.api.SubscribeEvent;
import net.neoforged.fml.common.EventBusSubscriber;
import net.neoforged.neoforge.event.tick.LevelTickEvent;

@EventBusSubscriber(modid = "eco")
public class NativeCollisionTest {

    private static final NativeCollision COLLISION =
        new NativeCollision();

    @SubscribeEvent
    public static void onLevelTick(LevelTickEvent.Post event) {
        Level level = event.getLevel();

        // Server-side only.
        if (level.isClientSide()) {
            return;
        }
        ServerLevel serverLevel = (ServerLevel) level;

        COLLISION.rebuild(serverLevel);

        // testing
        if (serverLevel.getGameTime() % 20 == 0) {
            for (Entity entity : serverLevel.getAllEntities()) {
                if (entity instanceof ServerPlayer player) {
                    int[] collisions = COLLISION.query(
                        player.getBoundingBox()
                    );

                    int count = 0;

                    for (int id : collisions) {
                        if (id != player.getId()) {
                            count++;
                        }
                    }

                    player.sendSystemMessage(
                        Component.literal(
                            "Native overlapping entities: " + count
                        )
                    );
                }
            }
        }
    }
}
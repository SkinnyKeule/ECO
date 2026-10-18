package com.optimization.eco;

import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.phys.AABB;

public class NativeCollision {
    static {
        System.load(
            "C:/Users/lucad/Documents/Coding/java/ECO-Mod/native/target/debug/native.dll"
        );
    }

    private static native void clearTree();
    private static native int[] queryNative(double minX, double minY, double minZ, double maxX, double maxY, double maxZ);
    private static native boolean upsertNative(int id, double minX, double minY, double minZ, double maxX, double maxY, double maxZ);
    private static native int deleteNative(int id);

    public void init() {
        NativeCollision.clearTree();
    }
    public int[] query(AABB bounds) {
        return NativeCollision.queryNative(bounds.minX, bounds.minY, bounds.minZ, bounds.maxX, bounds.maxY, bounds.maxZ);
    }
    public boolean upsert(int id, AABB bounds) {
        return NativeCollision.upsertNative(id, bounds.minX, bounds.minY, bounds.minZ, bounds.maxX, bounds.maxY, bounds.maxZ);
    }
    public int delete(int id) {
        return NativeCollision.deleteNative(id);
    }
    public void rebuild(ServerLevel level) {
        init();

        for (Entity entity : level.getAllEntities()) {
            AABB bounds = entity.getBoundingBox();

            upsert(entity.getId(), bounds);
        }
    }
}
package com.optimization.eco;

public class NativeCollision {
    static {
        System.load(
            "C:/Users/lucad/Documents/Coding/java/ECO-Mod/native/target/debug/native.dll"
        );
    }

    public static native void clearTree();
    public static native boolean insertEntity(int id, double minX, double minY, double minZ, double maxX, double maxY, double maxZ);
}

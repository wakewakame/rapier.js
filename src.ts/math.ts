import {RawVector, RawRotation} from "./raw";
export {
    acoshf,
    asinf,
    asinhf,
    atan2f,
    atanf,
    atanhf,
    cbrtf,
    ceilf,
    cosf,
    coshf,
    exp2f,
    exp10f,
    expf,
    expm1f,
    fabsf,
    floorf,
    fmodf,
    hypotf,
    log2f,
    log10f,
    logf,
    powf,
    sinf,
    sinhf,
    sqrtf,
    tanf,
    tanhf,
} from "./raw";

// #if DIM2
export interface Vector {
    x: number;
    y: number;
}

/**
 * A 2D vector.
 */
export class Vector2 implements Vector {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }
}

export class VectorOps {
    public static new(x: number, y: number): Vector {
        return new Vector2(x, y);
    }

    public static zeros(): Vector {
        return VectorOps.new(0.0, 0.0);
    }

    // FIXME: type ram: RawVector?
    public static fromRaw(raw: RawVector): Vector {
        if (!raw) return null;

        let res = VectorOps.new(raw.x, raw.y);
        raw.free();
        return res;
    }

    public static intoRaw(v: Vector): RawVector {
        return new RawVector(v.x, v.y);
    }

    public static copy(out: Vector, input: Vector) {
        out.x = input.x;
        out.y = input.y;
    }
}

/**
 * A rotation angle in radians.
 */
export type Rotation = number;

export class RotationOps {
    public static identity(): number {
        return 0.0;
    }

    public static fromRaw(raw: RawRotation): Rotation {
        if (!raw) return null;

        let res = raw.angle;
        raw.free();
        return res;
    }

    public static intoRaw(angle: Rotation): RawRotation {
        return RawRotation.fromAngle(angle);
    }
}

// #endif

// #if DIM3
export interface Vector {
    x: number;
    y: number;
    z: number;
}

/**
 * A 3D vector.
 */
export class Vector3 implements Vector {
    x: number;
    y: number;
    z: number;

    constructor(x: number, y: number, z: number) {
        this.x = x;
        this.y = y;
        this.z = z;
    }
}

export class VectorOps {
    public static new(x: number, y: number, z: number): Vector {
        return new Vector3(x, y, z);
    }

    public static intoRaw(v: Vector): RawVector {
        return new RawVector(v.x, v.y, v.z);
    }

    public static zeros(): Vector {
        return VectorOps.new(0.0, 0.0, 0.0);
    }

    // FIXME: type ram: RawVector?
    public static fromRaw(raw: RawVector): Vector {
        if (!raw) return null;

        let res = VectorOps.new(raw.x, raw.y, raw.z);
        raw.free();
        return res;
    }

    public static copy(out: Vector, input: Vector) {
        out.x = input.x;
        out.y = input.y;
        out.z = input.z;
    }
}

export interface Rotation {
    x: number;
    y: number;
    z: number;
    w: number;
}

/**
 * A quaternion.
 */
export class Quaternion implements Rotation {
    x: number;
    y: number;
    z: number;
    w: number;

    constructor(x: number, y: number, z: number, w: number) {
        this.x = x;
        this.y = y;
        this.z = z;
        this.w = w;
    }
}

export class RotationOps {
    public static identity(): Rotation {
        return new Quaternion(0.0, 0.0, 0.0, 1.0);
    }

    public static fromRaw(raw: RawRotation): Rotation {
        if (!raw) return null;

        let res = new Quaternion(raw.x, raw.y, raw.z, raw.w);
        raw.free();
        return res;
    }

    public static intoRaw(rot: Rotation): RawRotation {
        return new RawRotation(rot.x, rot.y, rot.z, rot.w);
    }

    public static copy(out: Rotation, input: Rotation) {
        out.x = input.x;
        out.y = input.y;
        out.z = input.z;
        out.w = input.w;
    }
}

// #endif

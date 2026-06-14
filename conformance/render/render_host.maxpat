{
 "patcher": {
  "fileversion": 1,
  "appversion": {
   "major": 9,
   "minor": 0,
   "revision": 0,
   "architecture": "x64",
   "modernui": 1
  },
  "classnamespace": "box",
  "rect": [
   100.0,
   100.0,
   1250.0,
   750.0
  ],
  "boxes": [
   {
    "box": {
     "id": "obj-1",
     "maxclass": "comment",
     "numinlets": 1,
     "numoutlets": 0,
     "patching_rect": [
      20,
      10,
      640,
      110
     ],
     "text": "GenExpr Conformance Render Host (v4)\nCapture happens INSIDE each gen~ (poke @ elapsed) \u2014 sample-aligned to patch t=0\nby construction. No record~, no arming.\n1. Open this patch; check Max console: all 82 gen~ must compile clean;\n   node.script autostarts and sizes 139 buffers to 4096 samples.\n2. Turn DSP ON (ezdac~), wait ~1 second, turn DSP OFF.\n   (Any sample rate works \u2014 dspstate~ reports the true rate to the runner.)\n3. Click [writewavs] \u2014 139 float32 WAVs land in conformance/golden/.\nRe-run? Close and reopen the patch first (fresh gen~ state)."
    }
   },
   {
    "box": {
     "id": "obj-2",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 2,
     "patching_rect": [
      20,
      140,
      330,
      22
     ],
     "text": "node.script render_runner.js @autostart 1 @watch 1",
     "outlettype": [
      "",
      ""
     ]
    }
   },
   {
    "box": {
     "id": "obj-3",
     "maxclass": "message",
     "numinlets": 2,
     "numoutlets": 1,
     "patching_rect": [
      20,
      180,
      80,
      22
     ],
     "text": "script start",
     "outlettype": [
      ""
     ]
    }
   },
   {
    "box": {
     "id": "obj-4",
     "maxclass": "message",
     "numinlets": 2,
     "numoutlets": 1,
     "patching_rect": [
      110,
      180,
      70,
      22
     ],
     "text": "writewavs",
     "outlettype": [
      ""
     ]
    }
   },
   {
    "box": {
     "id": "obj-5",
     "maxclass": "ezdac~",
     "numinlets": 2,
     "numoutlets": 0,
     "patching_rect": [
      380,
      140,
      45,
      45
     ]
    }
   },
   {
    "box": {
     "id": "obj-6",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 4,
     "patching_rect": [
      450,
      140,
      80,
      22
     ],
     "text": "dspstate~",
     "outlettype": [
      "int",
      "float",
      "int",
      "int"
     ]
    }
   },
   {
    "box": {
     "id": "obj-7",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      450,
      180,
      75,
      22
     ],
     "text": "prepend sr",
     "outlettype": [
      ""
     ]
    }
   },
   {
    "box": {
     "id": "obj-8",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 2,
     "patching_rect": [
      20,
      230,
      70,
      22
     ],
     "text": "route buf",
     "outlettype": [
      "",
      ""
     ]
    }
   },
   {
    "box": {
     "id": "obj-9",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 140,
     "patching_rect": [
      110,
      230,
      1000,
      22
     ],
     "text": "route cycle_440_ch0 dcblock_impulse_ch0 dcblock_step_ch0 delay_echo_ch0 delay_echo_ch1 delay_echo_ch2 history_counter_ch0 history_counter_ch1 history_read_after_write_ch0 phasor_incr_order_ch0 range_inverted_bounds_ch0 range_inverted_bounds_ch1 range_inverted_bounds_ch2 sah_latch_ch0 sah_latch_ch1 slide_step_ch0 triangle_duty_ch0 triangle_duty_ch1 triangle_duty_ch2 op_abs_ch0 op_absdiff_ch0 op_acos_ch0 op_acosh_ch0 op_add_ch0 op_and_ch0 op_and_ch1 op_and_ch2 op_and_ch3 op_asin_ch0 op_asinh_ch0 op_atan_ch0 op_atan2_ch0 op_atanh_ch0 op_atodb_ch0 op_bool_ch0 op_bool_ch1 op_bool_ch2 op_ceil_ch0 op_ceil_ch1 op_ceil_ch2 op_ceil_ch3 op_clamp_ch0 op_clip_ch0 op_cos_ch0 op_cosh_ch0 op_dbtoa_ch0 op_degrees_ch0 op_div_ch0 op_eq_ch0 op_eq_ch1 op_eq_ch2 op_exp_ch0 op_exp2_ch0 op_fixdenorm_ch0 op_fixnan_ch0 op_floor_ch0 op_floor_ch1 op_floor_ch2 op_floor_ch3 op_fold_ch0 op_fract_ch0 op_ftom_ch0 op_gt_ch0 op_gt_ch1 op_gt_ch2 op_gte_ch0 op_gte_ch1 op_gte_ch2 op_hypot_ch0 op_int_ch0 op_int_ch1 op_int_ch2 op_int_ch3 op_ln_ch0 op_log_ch0 op_log10_ch0 op_log2_ch0 op_lt_ch0 op_lt_ch1 op_lt_ch2 op_lte_ch0 op_lte_ch1 op_lte_ch2 op_max_ch0 op_min_ch0 op_mix_ch0 op_mod_ch0 op_mod_ch1 op_mod_ch2 op_mod_ch3 op_mstosamps_ch0 op_mtof_ch0 op_mul_ch0 op_neg_ch0 op_neq_ch0 op_neq_ch1 op_neq_ch2 op_not_ch0 op_not_ch1 op_or_ch0 op_or_ch1 op_or_ch2 op_or_ch3 op_pow_ch0 op_radians_ch0 op_rdiv_ch0 op_round_ch0 op_round_ch1 op_round_ch2 op_round_ch3 op_rsub_ch0 op_samplerate_ch0 op_sampstoms_ch0 op_scale_ch0 op_sign_ch0 op_sign_ch1 op_sign_ch2 op_sin_ch0 op_sinh_ch0 op_sqrt_ch0 op_sub_ch0 op_switch_ch0 op_switch_ch1 op_switch_ch2 op_tan_ch0 op_tanh_ch0 op_triangle_ch0 op_trunc_ch0 op_trunc_ch1 op_trunc_ch2 op_trunc_ch3 op_wrap_ch0 op_wrap_ch1 op_wrap_ch2 op_wrap_ch3 op_wrap_ch4 op_xor_ch0 op_xor_ch1 op_xor_ch2",
     "outlettype": [
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      ""
     ]
    }
   },
   {
    "box": {
     "id": "obj-10",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      290,
      230,
      22
     ],
     "text": "gen~ @title cycle_440",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// cycle_440.genexpr\n// Pure sine wave at A440 (standard tuning reference).\n// Tests that cycle() produces correct sine output at musical frequency.\n// Mono output: sin(2\u03c0\u00b7440\u00b7t) sampled at 48 kHz.\nout1 = cycle(440);\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer cycle_440_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke cycle_440_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-11",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      325,
      75,
      22
     ],
     "text": "buffer~ cycle_440_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-12",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      290,
      230,
      22
     ],
     "text": "gen~ @title dcblock_impulse",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// dcblock_impulse.genexpr\n// DISAMBIGUATION PROBE: distinguishes the two hypotheses for why real gen~\n// outputs silence for dcblock of a constant input (see dcblock_step):\n//   (a) lazy x1-init to the first input  \u2192 y = [0, -1, -0.9997, ...]\n//   (b) compiler constant-folding        \u2192 y = [1, -1+..., ...] (classic IR)\n// opengen implements (a); if the Max golden starts with 1.0, switch to (b)\n// semantics (genlib DCBlock form) and re-derive dcblock_step's explanation.\n//\n// gen~ compat: declared History, reads before write (see history_counter).\nHistory h(0);\nimp = eq(h, 0);\nout1 = dcblock(imp);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer dcblock_impulse_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke dcblock_impulse_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-13",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      325,
      75,
      22
     ],
     "text": "buffer~ dcblock_impulse_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-14",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      290,
      230,
      22
     ],
     "text": "gen~ @title dcblock_step",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// dcblock_step.genexpr\n// Step response of dcblock (one-pole highpass, coefficient 0.9997).\n// Constant input 1.0: first sample passes through, then exponential decay to ~0.\n// out1 = highpassed step (1.0, then decays toward 0).\n//\n// Analytic decay envelope: y[n] = 0.9997^(n-1) for n >= 1, y[0] = 1.0.\nout1 = dcblock(1.0);\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer dcblock_step_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke dcblock_step_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-15",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      325,
      75,
      22
     ],
     "text": "buffer~ dcblock_step_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-16",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      860,
      290,
      230,
      22
     ],
     "text": "gen~ @title delay_echo",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// delay_echo.genexpr\n// Impulse at sample 0 fed into a 64-sample delay line with three taps.\n// Tests delay_write + delay_read with linear interpolation.\n// out1 = tap at 1 sample (1-sample delayed impulse)\n// out2 = tap at 4 samples (4-sample delayed impulse)\n// out3 = tap at 16 samples (16-sample delayed impulse)\n//\n// Impulse generated via history counter:\n//   h[n] = h[n-1] + 1, imp[n] = (h[n] == 0) \u2192 fires at n=0 only.\n//\n// NOTE: gen~ requires declarations BEFORE expression statements in a codebox\n// (\"declarations must come before expressions\" \u2014 observed in Max 9,\n// 2026-06-10; matches docs/research/gen_docs/genexpr_ebnf.md program order).\n// gen~ also rejects the self-referential `h = history(h + 1)` shorthand\n// (\"variable h is not defined\") \u2014 feedback requires a declared History,\n// with reads before the write (see history_counter.genexpr).\n// opengen's parser is lenient on both.\nDelay d(64);\nHistory h(0);\nimp = eq(h, 0);\nd.write(imp);\nout1 = d.read(1);\nout2 = d.read(4);\nout3 = d.read(16);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer delay_echo_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke delay_echo_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer delay_echo_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke delay_echo_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer delay_echo_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke delay_echo_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-17",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      325,
      75,
      22
     ],
     "text": "buffer~ delay_echo_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-18",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      938,
      325,
      75,
      22
     ],
     "text": "buffer~ delay_echo_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-19",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      1016,
      325,
      75,
      22
     ],
     "text": "buffer~ delay_echo_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-20",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 2,
     "patching_rect": [
      20,
      390,
      230,
      22
     ],
     "text": "gen~ @title history_counter",
     "outlettype": [
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// history_counter.genexpr\n// Impulse-at-sample-zero via history counter.\n// h[n] = h[n-1] + 1, h[0] = 0 (zero-initialized history).\n// imp = (h == 0) \u2192 1.0 at sample 0 only, 0.0 thereafter.\n// out1 = impulse train (single impulse at origin).\n// out2 = counter value (monotonic integer sequence 0, 1, 2, ...).\n//\n// gen~ compat (observed Max 9, 2026-06-10): the self-referential shorthand\n// `h = history(h + 1)` is an opengen leniency \u2014 real gen~ rejects it\n// (\"variable h is not defined\"); feedback requires a declared History.\n// All reads precede the write: gen~ History reads AFTER an assignment see\n// the new value, opengen reads always see the previous sample. Keeping\n// reads first makes both engines agree.\nHistory h(0);\nimp = eq(h, 0);\nout1 = imp;\nout2 = h;\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer history_counter_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke history_counter_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer history_counter_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke history_counter_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-21",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      425,
      75,
      22
     ],
     "text": "buffer~ history_counter_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-22",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      98,
      425,
      75,
      22
     ],
     "text": "buffer~ history_counter_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-23",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      390,
      230,
      22
     ],
     "text": "gen~ @title history_read_after_write",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// history_read_after_write.genexpr\n// DIVERGENCE PROBE: does gen~ History act like a variable (read-after-assignment\n// sees the NEW value) or a dataflow port (reads always see the previous sample)?\n//\n// opengen: all History reads bind to the history OUTPUT port \u2192 always see\n//   the previous sample's value (dataflow semantics).\n// gen~: observed (2026-06-10) that History behaves like a variable \u2014 a read\n//   placed AFTER an assignment in the same sample body sees the NEW value.\n//\n// Prediction: gen~ golden = [1, 2, 3, 4, ...]; opengen renders [0, 1, 2, 3, ...].\n//\n// All authored conformance patches keep reads BEFORE the write so both engines\n// agree. This patch deliberately places the read AFTER the write to measure\n// the divergence.\nHistory h(0);\nh = h + 1;      // assignment: h becomes 1 (gen~ variable semantics)\nout1 = h;       // read after write: 1 (gen~) vs 0 (opengen)?\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer history_read_after_write_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke history_read_after_write_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-24",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      425,
      75,
      22
     ],
     "text": "buffer~ history_read_after_write_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-25",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      390,
      230,
      22
     ],
     "text": "gen~ @title phasor_incr_order",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// phasor_incr_order.genexpr\n// Settles the M1 `# Observed` wrap/increment-order question.\n// Odd frequency 997 Hz avoids bin-aligned coincidences at 48 kHz.\n// Output is mono: sawtooth ramp [0, 1) at 997 Hz.\nout1 = phasor(997);\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer phasor_incr_order_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke phasor_incr_order_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-26",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      425,
      75,
      22
     ],
     "text": "buffer~ phasor_incr_order_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-27",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      860,
      390,
      230,
      22
     ],
     "text": "gen~ @title range_inverted_bounds",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// range_inverted_bounds.genexpr\n// Tests clip/wrap/fold with inverted bounds (lo > hi).\n// Upgrades M1 Task 5 to `# Observed`.\n// clip(0.5, 1, 0): inverted bounds \u2192 pin to hi (0)\n// wrap(1.25, 1, 0): inverted \u2192 normalized to wrap(1.25, 0, 1) = 0.25\n// fold(1.25, 1, 0): inverted \u2192 normalized to fold(1.25, 0, 1) = 0.75\nout1 = clip(0.5, 1, 0);\nout2 = wrap(1.25, 1, 0);\nout3 = fold(1.25, 1, 0);\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer range_inverted_bounds_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke range_inverted_bounds_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer range_inverted_bounds_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke range_inverted_bounds_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer range_inverted_bounds_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke range_inverted_bounds_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-28",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      425,
      75,
      22
     ],
     "text": "buffer~ range_inverted_bounds_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-29",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      938,
      425,
      75,
      22
     ],
     "text": "buffer~ range_inverted_bounds_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-30",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      1016,
      425,
      75,
      22
     ],
     "text": "buffer~ range_inverted_bounds_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-31",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 2,
     "patching_rect": [
      20,
      490,
      230,
      22
     ],
     "text": "gen~ @title sah_latch",
     "outlettype": [
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// sah_latch.genexpr\n// Sample-and-hold and latch driven by history counter.\n// h = {0, 1, 2, 3, ..., 4095}\n//\n// sah: samples h when h crosses 2.5 (trigger at h=3).\n//   output: held=0 until sample 3, then 3 forever.\n//\n// latch: passes h when h is non-zero.\n//   output: h=0\u21920 (held), h=1\u21921, h=2\u21922, ...\n// gen~ compat: declared History + reads-before-write (see history_counter).\nHistory h(0);\nout1 = sah(h, h, 2.5);\nout2 = latch(h, h);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer sah_latch_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke sah_latch_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer sah_latch_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke sah_latch_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-32",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      525,
      75,
      22
     ],
     "text": "buffer~ sah_latch_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-33",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      98,
      525,
      75,
      22
     ],
     "text": "buffer~ sah_latch_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-34",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      490,
      230,
      22
     ],
     "text": "gen~ @title slide_step",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// slide_step.genexpr\n// Step response of slide (logarithmic smoother).\n// Step from 0 to 1 at sample 1 (sample 0 is 0).\n// Slide time constants: up=4, down=4 samples.\n// out1 = slewed step (asymptotic approach to 1.0).\n// gen~ compat: declared History + reads-before-write (see history_counter).\nHistory h(0);\nstep = switch(gt(h, 0), 1, 0);\nout1 = slide(step, 4, 4);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer slide_step_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke slide_step_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-35",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      525,
      75,
      22
     ],
     "text": "buffer~ slide_step_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-36",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      580,
      490,
      230,
      22
     ],
     "text": "gen~ @title triangle_duty",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// triangle_duty.genexpr\n// Triangle wave at 100 Hz with three duty cycles.\n// Tests triangle(phase, duty) with varying symmetry.\n// out1 = 25% duty (quick rise, slow fall)\n// out2 = 50% duty (symmetric triangle)\n// out3 = 75% duty (slow rise, quick fall)\np = phasor(100);\nout1 = triangle(p, 0.25);\nout2 = triangle(p, 0.5);\nout3 = triangle(p, 0.75);\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer triangle_duty_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke triangle_duty_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer triangle_duty_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke triangle_duty_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer triangle_duty_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke triangle_duty_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-37",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      525,
      75,
      22
     ],
     "text": "buffer~ triangle_duty_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-38",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      658,
      525,
      75,
      22
     ],
     "text": "buffer~ triangle_duty_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-39",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      736,
      525,
      75,
      22
     ],
     "text": "buffer~ triangle_duty_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-40",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      490,
      230,
      22
     ],
     "text": "gen~ @title op_abs",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `abs` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -2.5 + (2.5 - (-2.5)) * p;\nout1 = abs(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_abs_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_abs_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-41",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      525,
      75,
      22
     ],
     "text": "buffer~ op_abs_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-42",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      590,
      230,
      22
     ],
     "text": "gen~ @title op_absdiff",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `absdiff` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -3 + (3 - (-3)) * p;\nb = 3 + (-3 - (3)) * p;\nout1 = absdiff(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_absdiff_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_absdiff_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-43",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      625,
      75,
      22
     ],
     "text": "buffer~ op_absdiff_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-44",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      590,
      230,
      22
     ],
     "text": "gen~ @title op_acos",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `acos` (trig, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -1.0 + (1.0 - (-1.0)) * p;\nout1 = acos(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_acos_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_acos_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-45",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      625,
      75,
      22
     ],
     "text": "buffer~ op_acos_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-46",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      590,
      230,
      22
     ],
     "text": "gen~ @title op_acosh",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `acosh` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 1.1 + (5.0 - (1.1)) * p;\nout1 = acosh(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_acosh_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_acosh_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-47",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      625,
      75,
      22
     ],
     "text": "buffer~ op_acosh_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-48",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      590,
      230,
      22
     ],
     "text": "gen~ @title op_add",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `add` (math, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -2 + (2 - (-2)) * p;\nb = 2 + (-2 - (2)) * p;\nout1 = add(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_add_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_add_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-49",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      625,
      75,
      22
     ],
     "text": "buffer~ op_add_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-50",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 4,
     "patching_rect": [
      20,
      690,
      230,
      22
     ],
     "text": "gen~ @title op_and",
     "outlettype": [
      "signal",
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `and` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = and(2 + h*0, 3 + h*0);\nout2 = and(2 + h*0, 0 + h*0);\nout3 = and(0 + h*0, 3 + h*0);\nout4 = and(0 + h*0, 0 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 4,
         "outlettype": [
          "",
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_and_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_and_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_and_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_and_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_and_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_and_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-3",
         "maxclass": "newobj",
         "text": "buffer op_and_ch3",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          540.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-3",
         "maxclass": "newobj",
         "text": "poke op_and_ch3 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-4",
         "maxclass": "newobj",
         "text": "out 4",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "pk-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-3",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "go-4",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-51",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      725,
      75,
      22
     ],
     "text": "buffer~ op_and_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-52",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      98,
      725,
      75,
      22
     ],
     "text": "buffer~ op_and_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-53",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      176,
      725,
      75,
      22
     ],
     "text": "buffer~ op_and_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-54",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      254,
      725,
      75,
      22
     ],
     "text": "buffer~ op_and_ch3 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-55",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      690,
      230,
      22
     ],
     "text": "gen~ @title op_asin",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `asin` (trig, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -1.0 + (1.0 - (-1.0)) * p;\nout1 = asin(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_asin_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_asin_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-56",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      725,
      75,
      22
     ],
     "text": "buffer~ op_asin_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-57",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      690,
      230,
      22
     ],
     "text": "gen~ @title op_asinh",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `asinh` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -3.0 + (3.0 - (-3.0)) * p;\nout1 = asinh(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_asinh_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_asinh_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-58",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      725,
      75,
      22
     ],
     "text": "buffer~ op_asinh_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-59",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      690,
      230,
      22
     ],
     "text": "gen~ @title op_atan",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `atan` (trig, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -4.0 + (4.0 - (-4.0)) * p;\nout1 = atan(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_atan_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_atan_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-60",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      725,
      75,
      22
     ],
     "text": "buffer~ op_atan_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-61",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      790,
      230,
      22
     ],
     "text": "gen~ @title op_atan2",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `atan2` (trig, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -3 + (3 - (-3)) * p;\nb = 3 + (-3 - (3)) * p;\nout1 = atan2(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_atan2_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_atan2_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-62",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      825,
      75,
      22
     ],
     "text": "buffer~ op_atan2_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-63",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      790,
      230,
      22
     ],
     "text": "gen~ @title op_atanh",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `atanh` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -0.95 + (0.95 - (-0.95)) * p;\nout1 = atanh(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_atanh_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_atanh_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-64",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      825,
      75,
      22
     ],
     "text": "buffer~ op_atanh_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-65",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      790,
      230,
      22
     ],
     "text": "gen~ @title op_atodb",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `atodb` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 0.001 + (4.0 - (0.001)) * p;\nout1 = atodb(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_atodb_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_atodb_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-66",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      825,
      75,
      22
     ],
     "text": "buffer~ op_atodb_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-67",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      860,
      790,
      230,
      22
     ],
     "text": "gen~ @title op_bool",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `bool` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = bool(2.5 + h*0);\nout2 = bool(-2.5 + h*0);\nout3 = bool(0 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_bool_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_bool_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_bool_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_bool_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_bool_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_bool_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-68",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      825,
      75,
      22
     ],
     "text": "buffer~ op_bool_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-69",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      938,
      825,
      75,
      22
     ],
     "text": "buffer~ op_bool_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-70",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      1016,
      825,
      75,
      22
     ],
     "text": "buffer~ op_bool_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-71",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 4,
     "patching_rect": [
      20,
      890,
      230,
      22
     ],
     "text": "gen~ @title op_ceil",
     "outlettype": [
      "signal",
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `ceil` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = ceil(2.1 + h*0);\nout2 = ceil(2.7 + h*0);\nout3 = ceil(-1.1 + h*0);\nout4 = ceil(-1.9 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 4,
         "outlettype": [
          "",
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_ceil_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_ceil_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_ceil_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_ceil_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_ceil_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_ceil_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-3",
         "maxclass": "newobj",
         "text": "buffer op_ceil_ch3",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          540.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-3",
         "maxclass": "newobj",
         "text": "poke op_ceil_ch3 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-4",
         "maxclass": "newobj",
         "text": "out 4",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "pk-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-3",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "go-4",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-72",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      925,
      75,
      22
     ],
     "text": "buffer~ op_ceil_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-73",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      98,
      925,
      75,
      22
     ],
     "text": "buffer~ op_ceil_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-74",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      176,
      925,
      75,
      22
     ],
     "text": "buffer~ op_ceil_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-75",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      254,
      925,
      75,
      22
     ],
     "text": "buffer~ op_ceil_ch3 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-76",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      890,
      230,
      22
     ],
     "text": "gen~ @title op_clamp",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `clamp` (convert, arity 3).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nx = -2 + 4 * p;\nout1 = clamp(x, -1 + h*0, 1 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_clamp_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_clamp_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-77",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      925,
      75,
      22
     ],
     "text": "buffer~ op_clamp_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-78",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      890,
      230,
      22
     ],
     "text": "gen~ @title op_clip",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `clip` (range, arity 3).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nx = -2 + 4 * p;\nout1 = clip(x, -1 + h*0, 1 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_clip_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_clip_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-79",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      925,
      75,
      22
     ],
     "text": "buffer~ op_clip_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-80",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      890,
      230,
      22
     ],
     "text": "gen~ @title op_cos",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `cos` (trig, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -3.14159 + (3.14159 - (-3.14159)) * p;\nout1 = cos(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_cos_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_cos_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-81",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      925,
      75,
      22
     ],
     "text": "buffer~ op_cos_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-82",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      990,
      230,
      22
     ],
     "text": "gen~ @title op_cosh",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `cosh` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -3.0 + (3.0 - (-3.0)) * p;\nout1 = cosh(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_cosh_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_cosh_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-83",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1025,
      75,
      22
     ],
     "text": "buffer~ op_cosh_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-84",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      990,
      230,
      22
     ],
     "text": "gen~ @title op_dbtoa",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `dbtoa` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -60.0 + (12.0 - (-60.0)) * p;\nout1 = dbtoa(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_dbtoa_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_dbtoa_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-85",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1025,
      75,
      22
     ],
     "text": "buffer~ op_dbtoa_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-86",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      990,
      230,
      22
     ],
     "text": "gen~ @title op_degrees",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `degrees` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -6.0 + (6.0 - (-6.0)) * p;\nout1 = degrees(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_degrees_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_degrees_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-87",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1025,
      75,
      22
     ],
     "text": "buffer~ op_degrees_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-88",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      990,
      230,
      22
     ],
     "text": "gen~ @title op_div",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `div` (math, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -2 + (2 - (-2)) * p;\nb = 1 + (3 - (1)) * p;\nout1 = div(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_div_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_div_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-89",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1025,
      75,
      22
     ],
     "text": "buffer~ op_div_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-90",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      20,
      1090,
      230,
      22
     ],
     "text": "gen~ @title op_eq",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `eq` (compare, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = eq(2 + h*0, 2 + h*0);\nout2 = eq(2 + h*0, 3 + h*0);\nout3 = eq(3 + h*0, 2 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_eq_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_eq_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_eq_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_eq_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_eq_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_eq_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-91",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1125,
      75,
      22
     ],
     "text": "buffer~ op_eq_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-92",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      98,
      1125,
      75,
      22
     ],
     "text": "buffer~ op_eq_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-93",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      176,
      1125,
      75,
      22
     ],
     "text": "buffer~ op_eq_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-94",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1090,
      230,
      22
     ],
     "text": "gen~ @title op_exp",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `exp` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -4.0 + (4.0 - (-4.0)) * p;\nout1 = exp(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_exp_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_exp_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-95",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1125,
      75,
      22
     ],
     "text": "buffer~ op_exp_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-96",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1090,
      230,
      22
     ],
     "text": "gen~ @title op_exp2",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `exp2` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -4.0 + (4.0 - (-4.0)) * p;\nout1 = exp2(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_exp2_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_exp2_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-97",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1125,
      75,
      22
     ],
     "text": "buffer~ op_exp2_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-98",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1090,
      230,
      22
     ],
     "text": "gen~ @title op_fixdenorm",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `fixdenorm` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -3.0 + (3.0 - (-3.0)) * p;\nout1 = fixdenorm(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_fixdenorm_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_fixdenorm_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-99",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1125,
      75,
      22
     ],
     "text": "buffer~ op_fixdenorm_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-100",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1190,
      230,
      22
     ],
     "text": "gen~ @title op_fixnan",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `fixnan` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -3.0 + (3.0 - (-3.0)) * p;\nout1 = fixnan(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_fixnan_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_fixnan_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-101",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1225,
      75,
      22
     ],
     "text": "buffer~ op_fixnan_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-102",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 4,
     "patching_rect": [
      300,
      1190,
      230,
      22
     ],
     "text": "gen~ @title op_floor",
     "outlettype": [
      "signal",
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `floor` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = floor(2.3 + h*0);\nout2 = floor(2.9 + h*0);\nout3 = floor(-1.1 + h*0);\nout4 = floor(-1.9 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 4,
         "outlettype": [
          "",
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_floor_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_floor_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_floor_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_floor_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_floor_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_floor_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-3",
         "maxclass": "newobj",
         "text": "buffer op_floor_ch3",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          540.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-3",
         "maxclass": "newobj",
         "text": "poke op_floor_ch3 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-4",
         "maxclass": "newobj",
         "text": "out 4",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "pk-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-3",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "go-4",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-103",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1225,
      75,
      22
     ],
     "text": "buffer~ op_floor_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-104",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      378,
      1225,
      75,
      22
     ],
     "text": "buffer~ op_floor_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-105",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      456,
      1225,
      75,
      22
     ],
     "text": "buffer~ op_floor_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-106",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      534,
      1225,
      75,
      22
     ],
     "text": "buffer~ op_floor_ch3 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-107",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1190,
      230,
      22
     ],
     "text": "gen~ @title op_fold",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `fold` (range, arity 3).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nx = -2 + 5 * p;\nout1 = fold(x, 0 + h*0, 1 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_fold_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_fold_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-108",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1225,
      75,
      22
     ],
     "text": "buffer~ op_fold_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-109",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1190,
      230,
      22
     ],
     "text": "gen~ @title op_fract",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `fract` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 2.001 + (2.999 - (2.001)) * p;\nout1 = fract(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_fract_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_fract_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-110",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1225,
      75,
      22
     ],
     "text": "buffer~ op_fract_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-111",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1290,
      230,
      22
     ],
     "text": "gen~ @title op_ftom",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `ftom` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = 20 + (8000 - (20)) * p;\nb = 440 + (440 - (440)) * p;\nout1 = ftom(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_ftom_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_ftom_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-112",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1325,
      75,
      22
     ],
     "text": "buffer~ op_ftom_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-113",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      300,
      1290,
      230,
      22
     ],
     "text": "gen~ @title op_gt",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `gt` (compare, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = gt(3 + h*0, 1 + h*0);\nout2 = gt(1 + h*0, 3 + h*0);\nout3 = gt(2 + h*0, 2 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_gt_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_gt_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_gt_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_gt_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_gt_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_gt_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-114",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1325,
      75,
      22
     ],
     "text": "buffer~ op_gt_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-115",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      378,
      1325,
      75,
      22
     ],
     "text": "buffer~ op_gt_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-116",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      456,
      1325,
      75,
      22
     ],
     "text": "buffer~ op_gt_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-117",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      580,
      1290,
      230,
      22
     ],
     "text": "gen~ @title op_gte",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `gte` (compare, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = gte(3 + h*0, 1 + h*0);\nout2 = gte(1 + h*0, 3 + h*0);\nout3 = gte(2 + h*0, 2 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_gte_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_gte_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_gte_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_gte_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_gte_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_gte_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-118",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1325,
      75,
      22
     ],
     "text": "buffer~ op_gte_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-119",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      658,
      1325,
      75,
      22
     ],
     "text": "buffer~ op_gte_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-120",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      736,
      1325,
      75,
      22
     ],
     "text": "buffer~ op_gte_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-121",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1290,
      230,
      22
     ],
     "text": "gen~ @title op_hypot",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `hypot` (math, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -3 + (3 - (-3)) * p;\nb = 3 + (-3 - (3)) * p;\nout1 = hypot(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_hypot_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_hypot_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-122",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1325,
      75,
      22
     ],
     "text": "buffer~ op_hypot_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-123",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 4,
     "patching_rect": [
      20,
      1390,
      230,
      22
     ],
     "text": "gen~ @title op_int",
     "outlettype": [
      "signal",
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `int` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = int(2.3 + h*0);\nout2 = int(2.9 + h*0);\nout3 = int(-1.3 + h*0);\nout4 = int(-1.9 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 4,
         "outlettype": [
          "",
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_int_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_int_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_int_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_int_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_int_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_int_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-3",
         "maxclass": "newobj",
         "text": "buffer op_int_ch3",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          540.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-3",
         "maxclass": "newobj",
         "text": "poke op_int_ch3 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-4",
         "maxclass": "newobj",
         "text": "out 4",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "pk-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-3",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "go-4",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-124",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1425,
      75,
      22
     ],
     "text": "buffer~ op_int_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-125",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      98,
      1425,
      75,
      22
     ],
     "text": "buffer~ op_int_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-126",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      176,
      1425,
      75,
      22
     ],
     "text": "buffer~ op_int_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-127",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      254,
      1425,
      75,
      22
     ],
     "text": "buffer~ op_int_ch3 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-128",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1390,
      230,
      22
     ],
     "text": "gen~ @title op_ln",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `ln` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 0.01 + (8.0 - (0.01)) * p;\nout1 = ln(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_ln_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_ln_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-129",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1425,
      75,
      22
     ],
     "text": "buffer~ op_ln_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-130",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1390,
      230,
      22
     ],
     "text": "gen~ @title op_log",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `log` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 0.01 + (8.0 - (0.01)) * p;\nout1 = log(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_log_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_log_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-131",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1425,
      75,
      22
     ],
     "text": "buffer~ op_log_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-132",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1390,
      230,
      22
     ],
     "text": "gen~ @title op_log10",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `log10` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 0.01 + (8.0 - (0.01)) * p;\nout1 = log10(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_log10_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_log10_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-133",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1425,
      75,
      22
     ],
     "text": "buffer~ op_log10_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-134",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1490,
      230,
      22
     ],
     "text": "gen~ @title op_log2",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `log2` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 0.01 + (8.0 - (0.01)) * p;\nout1 = log2(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_log2_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_log2_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-135",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1525,
      75,
      22
     ],
     "text": "buffer~ op_log2_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-136",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      300,
      1490,
      230,
      22
     ],
     "text": "gen~ @title op_lt",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `lt` (compare, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = lt(1 + h*0, 3 + h*0);\nout2 = lt(3 + h*0, 1 + h*0);\nout3 = lt(2 + h*0, 2 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_lt_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_lt_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_lt_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_lt_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_lt_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_lt_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-137",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1525,
      75,
      22
     ],
     "text": "buffer~ op_lt_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-138",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      378,
      1525,
      75,
      22
     ],
     "text": "buffer~ op_lt_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-139",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      456,
      1525,
      75,
      22
     ],
     "text": "buffer~ op_lt_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-140",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      580,
      1490,
      230,
      22
     ],
     "text": "gen~ @title op_lte",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `lte` (compare, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = lte(1 + h*0, 3 + h*0);\nout2 = lte(3 + h*0, 1 + h*0);\nout3 = lte(2 + h*0, 2 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_lte_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_lte_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_lte_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_lte_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_lte_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_lte_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-141",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1525,
      75,
      22
     ],
     "text": "buffer~ op_lte_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-142",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      658,
      1525,
      75,
      22
     ],
     "text": "buffer~ op_lte_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-143",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      736,
      1525,
      75,
      22
     ],
     "text": "buffer~ op_lte_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-144",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1490,
      230,
      22
     ],
     "text": "gen~ @title op_max",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `max` (math, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -2 + (2 - (-2)) * p;\nb = 2 + (-2 - (2)) * p;\nout1 = max(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_max_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_max_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-145",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1525,
      75,
      22
     ],
     "text": "buffer~ op_max_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-146",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1590,
      230,
      22
     ],
     "text": "gen~ @title op_min",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `min` (math, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -2 + (2 - (-2)) * p;\nb = 2 + (-2 - (2)) * p;\nout1 = min(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_min_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_min_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-147",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1625,
      75,
      22
     ],
     "text": "buffer~ op_min_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-148",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1590,
      230,
      22
     ],
     "text": "gen~ @title op_mix",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `mix` (range, arity 3).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nt = 0 + 1 * p;\nout1 = mix(-1 + h*0, 2 + h*0, t);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_mix_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_mix_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-149",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1625,
      75,
      22
     ],
     "text": "buffer~ op_mix_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-150",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 4,
     "patching_rect": [
      580,
      1590,
      230,
      22
     ],
     "text": "gen~ @title op_mod",
     "outlettype": [
      "signal",
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `mod` (math, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = mod(2.3 + h*0, 2 + h*0);\nout2 = mod(-2.3 + h*0, 2 + h*0);\nout3 = mod(5.7 + h*0, 2 + h*0);\nout4 = mod(1.5 + h*0, 0.4 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 4,
         "outlettype": [
          "",
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_mod_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_mod_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_mod_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_mod_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_mod_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_mod_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-3",
         "maxclass": "newobj",
         "text": "buffer op_mod_ch3",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          540.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-3",
         "maxclass": "newobj",
         "text": "poke op_mod_ch3 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-4",
         "maxclass": "newobj",
         "text": "out 4",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "pk-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-3",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "go-4",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-151",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1625,
      75,
      22
     ],
     "text": "buffer~ op_mod_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-152",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      658,
      1625,
      75,
      22
     ],
     "text": "buffer~ op_mod_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-153",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      736,
      1625,
      75,
      22
     ],
     "text": "buffer~ op_mod_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-154",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      814,
      1625,
      75,
      22
     ],
     "text": "buffer~ op_mod_ch3 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-155",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1590,
      230,
      22
     ],
     "text": "gen~ @title op_mstosamps",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `mstosamps` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 0.0 + (100.0 - (0.0)) * p;\nout1 = mstosamps(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_mstosamps_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_mstosamps_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-156",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1625,
      75,
      22
     ],
     "text": "buffer~ op_mstosamps_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-157",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1690,
      230,
      22
     ],
     "text": "gen~ @title op_mtof",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `mtof` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = 0 + (127 - (0)) * p;\nb = 440 + (440 - (440)) * p;\nout1 = mtof(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_mtof_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_mtof_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-158",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1725,
      75,
      22
     ],
     "text": "buffer~ op_mtof_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-159",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1690,
      230,
      22
     ],
     "text": "gen~ @title op_mul",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `mul` (math, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -2 + (2 - (-2)) * p;\nb = 2 + (-2 - (2)) * p;\nout1 = mul(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_mul_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_mul_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-160",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1725,
      75,
      22
     ],
     "text": "buffer~ op_mul_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-161",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1690,
      230,
      22
     ],
     "text": "gen~ @title op_neg",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `neg` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -2.5 + (2.5 - (-2.5)) * p;\nout1 = neg(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_neg_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_neg_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-162",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1725,
      75,
      22
     ],
     "text": "buffer~ op_neg_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-163",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      860,
      1690,
      230,
      22
     ],
     "text": "gen~ @title op_neq",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `neq` (compare, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = neq(2 + h*0, 2 + h*0);\nout2 = neq(2 + h*0, 3 + h*0);\nout3 = neq(3 + h*0, 2 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_neq_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_neq_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_neq_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_neq_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_neq_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_neq_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-164",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1725,
      75,
      22
     ],
     "text": "buffer~ op_neq_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-165",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      938,
      1725,
      75,
      22
     ],
     "text": "buffer~ op_neq_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-166",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      1016,
      1725,
      75,
      22
     ],
     "text": "buffer~ op_neq_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-167",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 2,
     "patching_rect": [
      20,
      1790,
      230,
      22
     ],
     "text": "gen~ @title op_not",
     "outlettype": [
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `not` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = not(2.5 + h*0);\nout2 = not(0 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_not_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_not_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_not_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_not_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-168",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1825,
      75,
      22
     ],
     "text": "buffer~ op_not_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-169",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      98,
      1825,
      75,
      22
     ],
     "text": "buffer~ op_not_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-170",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 4,
     "patching_rect": [
      300,
      1790,
      230,
      22
     ],
     "text": "gen~ @title op_or",
     "outlettype": [
      "signal",
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `or` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = or(2 + h*0, 3 + h*0);\nout2 = or(2 + h*0, 0 + h*0);\nout3 = or(0 + h*0, 3 + h*0);\nout4 = or(0 + h*0, 0 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 4,
         "outlettype": [
          "",
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_or_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_or_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_or_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_or_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_or_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_or_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-3",
         "maxclass": "newobj",
         "text": "buffer op_or_ch3",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          540.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-3",
         "maxclass": "newobj",
         "text": "poke op_or_ch3 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-4",
         "maxclass": "newobj",
         "text": "out 4",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "pk-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-3",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "go-4",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-171",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1825,
      75,
      22
     ],
     "text": "buffer~ op_or_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-172",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      378,
      1825,
      75,
      22
     ],
     "text": "buffer~ op_or_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-173",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      456,
      1825,
      75,
      22
     ],
     "text": "buffer~ op_or_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-174",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      534,
      1825,
      75,
      22
     ],
     "text": "buffer~ op_or_ch3 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-175",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1790,
      230,
      22
     ],
     "text": "gen~ @title op_pow",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `pow` (math, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = 0.1 + (3 - (0.1)) * p;\nb = -2 + (3 - (-2)) * p;\nout1 = pow(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_pow_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_pow_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-176",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1825,
      75,
      22
     ],
     "text": "buffer~ op_pow_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-177",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1790,
      230,
      22
     ],
     "text": "gen~ @title op_radians",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `radians` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -360.0 + (360.0 - (-360.0)) * p;\nout1 = radians(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_radians_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_radians_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-178",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1825,
      75,
      22
     ],
     "text": "buffer~ op_radians_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-179",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1890,
      230,
      22
     ],
     "text": "gen~ @title op_rdiv",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `rdiv` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = 1 + (3 - (1)) * p;\nb = -2 + (2 - (-2)) * p;\nout1 = rdiv(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_rdiv_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_rdiv_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-180",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1925,
      75,
      22
     ],
     "text": "buffer~ op_rdiv_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-181",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 4,
     "patching_rect": [
      300,
      1890,
      230,
      22
     ],
     "text": "gen~ @title op_round",
     "outlettype": [
      "signal",
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `round` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = round(2.3 + h*0, 1 + h*0);\nout2 = round(2.7 + h*0, 1 + h*0);\nout3 = round(-1.3 + h*0, 1 + h*0);\nout4 = round(5.2 + h*0, 2 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 4,
         "outlettype": [
          "",
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_round_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_round_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_round_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_round_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_round_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_round_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-3",
         "maxclass": "newobj",
         "text": "buffer op_round_ch3",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          540.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-3",
         "maxclass": "newobj",
         "text": "poke op_round_ch3 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-4",
         "maxclass": "newobj",
         "text": "out 4",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "pk-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-3",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "go-4",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-182",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1925,
      75,
      22
     ],
     "text": "buffer~ op_round_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-183",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      378,
      1925,
      75,
      22
     ],
     "text": "buffer~ op_round_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-184",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      456,
      1925,
      75,
      22
     ],
     "text": "buffer~ op_round_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-185",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      534,
      1925,
      75,
      22
     ],
     "text": "buffer~ op_round_ch3 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-186",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1890,
      230,
      22
     ],
     "text": "gen~ @title op_rsub",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `rsub` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -2 + (2 - (-2)) * p;\nb = 2 + (-2 - (2)) * p;\nout1 = rsub(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_rsub_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_rsub_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-187",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      1925,
      75,
      22
     ],
     "text": "buffer~ op_rsub_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-188",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1890,
      230,
      22
     ],
     "text": "gen~ @title op_samplerate",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `samplerate` (samplerate, arity 0).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = samplerate + h * 0;\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_samplerate_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_samplerate_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-189",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1925,
      75,
      22
     ],
     "text": "buffer~ op_samplerate_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-190",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      1990,
      230,
      22
     ],
     "text": "gen~ @title op_sampstoms",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `sampstoms` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 0.0 + (4800.0 - (0.0)) * p;\nout1 = sampstoms(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_sampstoms_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_sampstoms_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-191",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      2025,
      75,
      22
     ],
     "text": "buffer~ op_sampstoms_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-192",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      1990,
      230,
      22
     ],
     "text": "gen~ @title op_scale",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `scale` (range, arity 5).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nx = 0 + 1 * p;\nout1 = scale(x, 0 + h*0, 1 + h*0, -5 + h*0, 5 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_scale_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_scale_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-193",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      2025,
      75,
      22
     ],
     "text": "buffer~ op_scale_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-194",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      580,
      1990,
      230,
      22
     ],
     "text": "gen~ @title op_sign",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `sign` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = sign(2.5 + h*0);\nout2 = sign(-2.5 + h*0);\nout3 = sign(0 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_sign_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_sign_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_sign_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_sign_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_sign_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_sign_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-195",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      2025,
      75,
      22
     ],
     "text": "buffer~ op_sign_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-196",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      658,
      2025,
      75,
      22
     ],
     "text": "buffer~ op_sign_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-197",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      736,
      2025,
      75,
      22
     ],
     "text": "buffer~ op_sign_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-198",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      1990,
      230,
      22
     ],
     "text": "gen~ @title op_sin",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `sin` (trig, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -3.14159 + (3.14159 - (-3.14159)) * p;\nout1 = sin(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_sin_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_sin_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-199",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      2025,
      75,
      22
     ],
     "text": "buffer~ op_sin_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-200",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      2090,
      230,
      22
     ],
     "text": "gen~ @title op_sinh",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `sinh` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -3.0 + (3.0 - (-3.0)) * p;\nout1 = sinh(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_sinh_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_sinh_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-201",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      2125,
      75,
      22
     ],
     "text": "buffer~ op_sinh_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-202",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      2090,
      230,
      22
     ],
     "text": "gen~ @title op_sqrt",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `sqrt` (math, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = 0.0 + (4.0 - (0.0)) * p;\nout1 = sqrt(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_sqrt_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_sqrt_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-203",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      2125,
      75,
      22
     ],
     "text": "buffer~ op_sqrt_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-204",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      2090,
      230,
      22
     ],
     "text": "gen~ @title op_sub",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `sub` (math, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = -2 + (2 - (-2)) * p;\nb = 2 + (-2 - (2)) * p;\nout1 = sub(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_sub_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_sub_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-205",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      2125,
      75,
      22
     ],
     "text": "buffer~ op_sub_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-206",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      860,
      2090,
      230,
      22
     ],
     "text": "gen~ @title op_switch",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `switch` (convert, arity 3).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = switch(1 + h*0, 10 + h*0, 20 + h*0);\nout2 = switch(0 + h*0, 10 + h*0, 20 + h*0);\nout3 = switch(-1 + h*0, 10 + h*0, 20 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_switch_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_switch_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_switch_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_switch_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_switch_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_switch_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-207",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      2125,
      75,
      22
     ],
     "text": "buffer~ op_switch_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-208",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      938,
      2125,
      75,
      22
     ],
     "text": "buffer~ op_switch_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-209",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      1016,
      2125,
      75,
      22
     ],
     "text": "buffer~ op_switch_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-210",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      2190,
      230,
      22
     ],
     "text": "gen~ @title op_tan",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `tan` (trig, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -1.4 + (1.4 - (-1.4)) * p;\nout1 = tan(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_tan_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_tan_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-211",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      2225,
      75,
      22
     ],
     "text": "buffer~ op_tan_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-212",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      2190,
      230,
      22
     ],
     "text": "gen~ @title op_tanh",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `tanh` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\ns = -3.0 + (3.0 - (-3.0)) * p;\nout1 = tanh(s);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_tanh_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_tanh_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-213",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      2225,
      75,
      22
     ],
     "text": "buffer~ op_tanh_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-214",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      2190,
      230,
      22
     ],
     "text": "gen~ @title op_triangle",
     "outlettype": [
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance sweep for `triangle` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\na = 0 + (1 - (0)) * p;\nb = 0.3 + (0.7 - (0.3)) * p;\nout1 = triangle(a, b);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_triangle_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_triangle_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-215",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
      2225,
      75,
      22
     ],
     "text": "buffer~ op_triangle_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-216",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 4,
     "patching_rect": [
      860,
      2190,
      230,
      22
     ],
     "text": "gen~ @title op_trunc",
     "outlettype": [
      "signal",
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `trunc` (convert, arity 1).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = trunc(2.3 + h*0);\nout2 = trunc(2.9 + h*0);\nout3 = trunc(-1.3 + h*0);\nout4 = trunc(-1.9 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 4,
         "outlettype": [
          "",
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_trunc_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_trunc_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_trunc_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_trunc_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_trunc_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_trunc_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-3",
         "maxclass": "newobj",
         "text": "buffer op_trunc_ch3",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          540.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-3",
         "maxclass": "newobj",
         "text": "poke op_trunc_ch3 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-4",
         "maxclass": "newobj",
         "text": "out 4",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "pk-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-3",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "go-4",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-217",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      860,
      2225,
      75,
      22
     ],
     "text": "buffer~ op_trunc_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-218",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      938,
      2225,
      75,
      22
     ],
     "text": "buffer~ op_trunc_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-219",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      1016,
      2225,
      75,
      22
     ],
     "text": "buffer~ op_trunc_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-220",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      1094,
      2225,
      75,
      22
     ],
     "text": "buffer~ op_trunc_ch3 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-221",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 5,
     "patching_rect": [
      20,
      2290,
      230,
      22
     ],
     "text": "gen~ @title op_wrap",
     "outlettype": [
      "signal",
      "signal",
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `wrap` (range, arity 3).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = wrap(0.3 + h*0, 0 + h*0, 1 + h*0);\nout2 = wrap(0.7 + h*0, 0 + h*0, 1 + h*0);\nout3 = wrap(1.3 + h*0, 0 + h*0, 1 + h*0);\nout4 = wrap(2.6 + h*0, 0 + h*0, 1 + h*0);\nout5 = wrap(-0.4 + h*0, 0 + h*0, 1 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 5,
         "outlettype": [
          "",
          "",
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_wrap_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_wrap_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_wrap_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_wrap_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_wrap_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_wrap_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-3",
         "maxclass": "newobj",
         "text": "buffer op_wrap_ch3",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          540.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-3",
         "maxclass": "newobj",
         "text": "poke op_wrap_ch3 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-4",
         "maxclass": "newobj",
         "text": "out 4",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          540.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-4",
         "maxclass": "newobj",
         "text": "buffer op_wrap_ch4",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          710.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-4",
         "maxclass": "newobj",
         "text": "poke op_wrap_ch4 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          710.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-5",
         "maxclass": "newobj",
         "text": "out 5",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          710.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "pk-3",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-3",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          3
         ],
         "destination": [
          "go-4",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          4
         ],
         "destination": [
          "pk-4",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-4",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          4
         ],
         "destination": [
          "go-5",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-222",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      2325,
      75,
      22
     ],
     "text": "buffer~ op_wrap_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-223",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      98,
      2325,
      75,
      22
     ],
     "text": "buffer~ op_wrap_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-224",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      176,
      2325,
      75,
      22
     ],
     "text": "buffer~ op_wrap_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-225",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      254,
      2325,
      75,
      22
     ],
     "text": "buffer~ op_wrap_ch3 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-226",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      332,
      2325,
      75,
      22
     ],
     "text": "buffer~ op_wrap_ch4 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-227",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      300,
      2290,
      230,
      22
     ],
     "text": "gen~ @title op_xor",
     "outlettype": [
      "signal",
      "signal",
      "signal"
     ],
     "patcher": {
      "fileversion": 1,
      "appversion": {
       "major": 9,
       "minor": 0,
       "revision": 0,
       "architecture": "x64",
       "modernui": 1
      },
      "classnamespace": "dsp.gen",
      "rect": [
       100.0,
       100.0,
       760.0,
       520.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// AUTO-GENERATED by tools/gen_op_sweeps.py \u2014 do not edit by hand.\n// Per-operator conformance points for `xor` (convert, arity 2).\n// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n// constant folder. Output = raw operator result.\nHistory h(0);\np = h / 4095;\nout1 = xor(2 + h*0, 3 + h*0);\nout2 = xor(2 + h*0, 0 + h*0);\nout3 = xor(0 + h*0, 0 + h*0);\nh = h + 1;\n",
         "numinlets": 0,
         "numoutlets": 3,
         "outlettype": [
          "",
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          30.0,
          480.0,
          300.0
         ]
        }
       },
       {
        "box": {
         "id": "el-1",
         "maxclass": "newobj",
         "text": "elapsed",
         "numinlets": 0,
         "numoutlets": 1,
         "outlettype": [
          ""
         ],
         "patching_rect": [
          550.0,
          30.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-0",
         "maxclass": "newobj",
         "text": "buffer op_xor_ch0",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          30.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-0",
         "maxclass": "newobj",
         "text": "poke op_xor_ch0 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-1",
         "maxclass": "newobj",
         "text": "buffer op_xor_ch1",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          200.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-1",
         "maxclass": "newobj",
         "text": "poke op_xor_ch1 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-2",
         "maxclass": "newobj",
         "text": "out 2",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          200.0,
          440.0,
          60.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "buf-2",
         "maxclass": "newobj",
         "text": "buffer op_xor_ch2",
         "numinlets": 0,
         "numoutlets": 2,
         "outlettype": [
          "",
          ""
         ],
         "patching_rect": [
          370.0,
          360.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "pk-2",
         "maxclass": "newobj",
         "text": "poke op_xor_ch2 0",
         "numinlets": 2,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          400.0,
          150.0,
          22.0
         ]
        }
       },
       {
        "box": {
         "id": "go-3",
         "maxclass": "newobj",
         "text": "out 3",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          370.0,
          440.0,
          60.0,
          22.0
         ]
        }
       }
      ],
      "lines": [
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "pk-0",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-0",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          0
         ],
         "destination": [
          "go-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "pk-1",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-1",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          1
         ],
         "destination": [
          "go-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "pk-2",
          0
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "el-1",
          0
         ],
         "destination": [
          "pk-2",
          1
         ]
        }
       },
       {
        "patchline": {
         "source": [
          "cb-1",
          2
         ],
         "destination": [
          "go-3",
          0
         ]
        }
       }
      ]
     }
    }
   },
   {
    "box": {
     "id": "obj-228",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
      2325,
      75,
      22
     ],
     "text": "buffer~ op_xor_ch0 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-229",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      378,
      2325,
      75,
      22
     ],
     "text": "buffer~ op_xor_ch1 86",
     "outlettype": [
      "float"
     ]
    }
   },
   {
    "box": {
     "id": "obj-230",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      456,
      2325,
      75,
      22
     ],
     "text": "buffer~ op_xor_ch2 86",
     "outlettype": [
      "float"
     ]
    }
   }
  ],
  "lines": [
   {
    "patchline": {
     "source": [
      "obj-3",
      0
     ],
     "destination": [
      "obj-2",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-4",
      0
     ],
     "destination": [
      "obj-2",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-6",
      1
     ],
     "destination": [
      "obj-7",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-7",
      0
     ],
     "destination": [
      "obj-2",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-2",
      0
     ],
     "destination": [
      "obj-8",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-8",
      0
     ],
     "destination": [
      "obj-9",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      0
     ],
     "destination": [
      "obj-11",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      1
     ],
     "destination": [
      "obj-13",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      2
     ],
     "destination": [
      "obj-15",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      3
     ],
     "destination": [
      "obj-17",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      4
     ],
     "destination": [
      "obj-18",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      5
     ],
     "destination": [
      "obj-19",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      6
     ],
     "destination": [
      "obj-21",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      7
     ],
     "destination": [
      "obj-22",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      8
     ],
     "destination": [
      "obj-24",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      9
     ],
     "destination": [
      "obj-26",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      10
     ],
     "destination": [
      "obj-28",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      11
     ],
     "destination": [
      "obj-29",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      12
     ],
     "destination": [
      "obj-30",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      13
     ],
     "destination": [
      "obj-32",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      14
     ],
     "destination": [
      "obj-33",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      15
     ],
     "destination": [
      "obj-35",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      16
     ],
     "destination": [
      "obj-37",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      17
     ],
     "destination": [
      "obj-38",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      18
     ],
     "destination": [
      "obj-39",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      19
     ],
     "destination": [
      "obj-41",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      20
     ],
     "destination": [
      "obj-43",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      21
     ],
     "destination": [
      "obj-45",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      22
     ],
     "destination": [
      "obj-47",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      23
     ],
     "destination": [
      "obj-49",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      24
     ],
     "destination": [
      "obj-51",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      25
     ],
     "destination": [
      "obj-52",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      26
     ],
     "destination": [
      "obj-53",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      27
     ],
     "destination": [
      "obj-54",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      28
     ],
     "destination": [
      "obj-56",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      29
     ],
     "destination": [
      "obj-58",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      30
     ],
     "destination": [
      "obj-60",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      31
     ],
     "destination": [
      "obj-62",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      32
     ],
     "destination": [
      "obj-64",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      33
     ],
     "destination": [
      "obj-66",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      34
     ],
     "destination": [
      "obj-68",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      35
     ],
     "destination": [
      "obj-69",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      36
     ],
     "destination": [
      "obj-70",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      37
     ],
     "destination": [
      "obj-72",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      38
     ],
     "destination": [
      "obj-73",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      39
     ],
     "destination": [
      "obj-74",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      40
     ],
     "destination": [
      "obj-75",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      41
     ],
     "destination": [
      "obj-77",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      42
     ],
     "destination": [
      "obj-79",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      43
     ],
     "destination": [
      "obj-81",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      44
     ],
     "destination": [
      "obj-83",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      45
     ],
     "destination": [
      "obj-85",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      46
     ],
     "destination": [
      "obj-87",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      47
     ],
     "destination": [
      "obj-89",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      48
     ],
     "destination": [
      "obj-91",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      49
     ],
     "destination": [
      "obj-92",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      50
     ],
     "destination": [
      "obj-93",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      51
     ],
     "destination": [
      "obj-95",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      52
     ],
     "destination": [
      "obj-97",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      53
     ],
     "destination": [
      "obj-99",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      54
     ],
     "destination": [
      "obj-101",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      55
     ],
     "destination": [
      "obj-103",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      56
     ],
     "destination": [
      "obj-104",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      57
     ],
     "destination": [
      "obj-105",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      58
     ],
     "destination": [
      "obj-106",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      59
     ],
     "destination": [
      "obj-108",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      60
     ],
     "destination": [
      "obj-110",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      61
     ],
     "destination": [
      "obj-112",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      62
     ],
     "destination": [
      "obj-114",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      63
     ],
     "destination": [
      "obj-115",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      64
     ],
     "destination": [
      "obj-116",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      65
     ],
     "destination": [
      "obj-118",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      66
     ],
     "destination": [
      "obj-119",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      67
     ],
     "destination": [
      "obj-120",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      68
     ],
     "destination": [
      "obj-122",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      69
     ],
     "destination": [
      "obj-124",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      70
     ],
     "destination": [
      "obj-125",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      71
     ],
     "destination": [
      "obj-126",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      72
     ],
     "destination": [
      "obj-127",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      73
     ],
     "destination": [
      "obj-129",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      74
     ],
     "destination": [
      "obj-131",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      75
     ],
     "destination": [
      "obj-133",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      76
     ],
     "destination": [
      "obj-135",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      77
     ],
     "destination": [
      "obj-137",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      78
     ],
     "destination": [
      "obj-138",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      79
     ],
     "destination": [
      "obj-139",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      80
     ],
     "destination": [
      "obj-141",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      81
     ],
     "destination": [
      "obj-142",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      82
     ],
     "destination": [
      "obj-143",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      83
     ],
     "destination": [
      "obj-145",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      84
     ],
     "destination": [
      "obj-147",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      85
     ],
     "destination": [
      "obj-149",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      86
     ],
     "destination": [
      "obj-151",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      87
     ],
     "destination": [
      "obj-152",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      88
     ],
     "destination": [
      "obj-153",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      89
     ],
     "destination": [
      "obj-154",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      90
     ],
     "destination": [
      "obj-156",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      91
     ],
     "destination": [
      "obj-158",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      92
     ],
     "destination": [
      "obj-160",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      93
     ],
     "destination": [
      "obj-162",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      94
     ],
     "destination": [
      "obj-164",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      95
     ],
     "destination": [
      "obj-165",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      96
     ],
     "destination": [
      "obj-166",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      97
     ],
     "destination": [
      "obj-168",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      98
     ],
     "destination": [
      "obj-169",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      99
     ],
     "destination": [
      "obj-171",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      100
     ],
     "destination": [
      "obj-172",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      101
     ],
     "destination": [
      "obj-173",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      102
     ],
     "destination": [
      "obj-174",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      103
     ],
     "destination": [
      "obj-176",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      104
     ],
     "destination": [
      "obj-178",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      105
     ],
     "destination": [
      "obj-180",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      106
     ],
     "destination": [
      "obj-182",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      107
     ],
     "destination": [
      "obj-183",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      108
     ],
     "destination": [
      "obj-184",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      109
     ],
     "destination": [
      "obj-185",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      110
     ],
     "destination": [
      "obj-187",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      111
     ],
     "destination": [
      "obj-189",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      112
     ],
     "destination": [
      "obj-191",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      113
     ],
     "destination": [
      "obj-193",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      114
     ],
     "destination": [
      "obj-195",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      115
     ],
     "destination": [
      "obj-196",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      116
     ],
     "destination": [
      "obj-197",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      117
     ],
     "destination": [
      "obj-199",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      118
     ],
     "destination": [
      "obj-201",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      119
     ],
     "destination": [
      "obj-203",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      120
     ],
     "destination": [
      "obj-205",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      121
     ],
     "destination": [
      "obj-207",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      122
     ],
     "destination": [
      "obj-208",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      123
     ],
     "destination": [
      "obj-209",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      124
     ],
     "destination": [
      "obj-211",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      125
     ],
     "destination": [
      "obj-213",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      126
     ],
     "destination": [
      "obj-215",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      127
     ],
     "destination": [
      "obj-217",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      128
     ],
     "destination": [
      "obj-218",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      129
     ],
     "destination": [
      "obj-219",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      130
     ],
     "destination": [
      "obj-220",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      131
     ],
     "destination": [
      "obj-222",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      132
     ],
     "destination": [
      "obj-223",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      133
     ],
     "destination": [
      "obj-224",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      134
     ],
     "destination": [
      "obj-225",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      135
     ],
     "destination": [
      "obj-226",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      136
     ],
     "destination": [
      "obj-228",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      137
     ],
     "destination": [
      "obj-229",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-9",
      138
     ],
     "destination": [
      "obj-230",
      0
     ]
    }
   }
  ],
  "dependency_cache": [],
  "autosave": 0
 }
}

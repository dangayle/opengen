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
     "text": "GenExpr Conformance Render Host (v4)\nCapture happens INSIDE each gen~ (poke @ elapsed) \u2014 sample-aligned to patch t=0\nby construction. No record~, no arming.\n1. Open this patch; check Max console: all 9 gen~ must compile clean;\n   node.script autostarts and sizes 17 buffers to 4096 samples.\n2. Turn DSP ON (ezdac~), wait ~1 second, turn DSP OFF.\n3. Click [writewavs] \u2014 17 float32 WAVs land in conformance/golden/.\nRe-run? Close and reopen the patch first (fresh gen~ state)."
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
     "id": "obj-7",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 19,
     "patching_rect": [
      110,
      230,
      1000,
      22
     ],
     "text": "route cycle_440_ch0 dcblock_impulse_ch0 dcblock_step_ch0 delay_echo_ch0 delay_echo_ch1 delay_echo_ch2 history_counter_ch0 history_counter_ch1 phasor_incr_order_ch0 range_inverted_bounds_ch0 range_inverted_bounds_ch1 range_inverted_bounds_ch2 sah_latch_ch0 sah_latch_ch1 slide_step_ch0 triangle_duty_ch0 triangle_duty_ch1 triangle_duty_ch2",
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
      ""
     ]
    }
   },
   {
    "box": {
     "id": "obj-8",
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
     "id": "obj-9",
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
     "id": "obj-10",
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
     "id": "obj-11",
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
     "id": "obj-12",
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
     "id": "obj-13",
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
     "id": "obj-14",
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
     "id": "obj-15",
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
     "id": "obj-16",
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
     "id": "obj-17",
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
     "id": "obj-18",
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
     "id": "obj-19",
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
     "id": "obj-20",
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
     "id": "obj-21",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
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
     "id": "obj-22",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
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
     "id": "obj-23",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      580,
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
     "id": "obj-24",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      580,
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
     "id": "obj-25",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      658,
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
     "id": "obj-26",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      736,
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
     "id": "obj-27",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 2,
     "patching_rect": [
      860,
      390,
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
     "text": "buffer~ sah_latch_ch0 86",
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
     "text": "buffer~ sah_latch_ch1 86",
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
      20,
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
     "id": "obj-31",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
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
     "id": "obj-32",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      300,
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
     "id": "obj-33",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      300,
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
     "id": "obj-34",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      378,
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
     "id": "obj-35",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      456,
      525,
      75,
      22
     ],
     "text": "buffer~ triangle_duty_ch2 86",
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
      "obj-2",
      0
     ],
     "destination": [
      "obj-6",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-6",
      0
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
      "obj-9",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-7",
      1
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
      "obj-7",
      2
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
      "obj-7",
      3
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
      "obj-7",
      4
     ],
     "destination": [
      "obj-16",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-7",
      5
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
      "obj-7",
      6
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
      "obj-7",
      7
     ],
     "destination": [
      "obj-20",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-7",
      8
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
      "obj-7",
      9
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
      "obj-7",
      10
     ],
     "destination": [
      "obj-25",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-7",
      11
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
      "obj-7",
      12
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
      "obj-7",
      13
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
      "obj-7",
      14
     ],
     "destination": [
      "obj-31",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-7",
      15
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
      "obj-7",
      16
     ],
     "destination": [
      "obj-34",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-7",
      17
     ],
     "destination": [
      "obj-35",
      0
     ]
    }
   }
  ],
  "dependency_cache": [],
  "autosave": 0
 }
}

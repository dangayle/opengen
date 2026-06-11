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
   1200.0,
   800.0
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
      620,
      120
     ],
     "text": "GenExpr Conformance Render Host (v3)\nGenExpr sources are EMBEDDED (codebox in each gen~) \u2014 nothing to resolve.\n1. Open this patch; check Max console: all 9 gen~ must compile clean.\n2. node.script autostarts (or click [script start]); console shows buffer sizing.\n3. Click [arm] with DSP OFF.\n4. Turn DSP ON (ezdac~), wait 1 second, turn DSP OFF.\n5. Click [writewavs] \u2014 17 WAVs land in conformance/golden/.\nRe-run? Close and reopen the patch first (fresh gen~ state)."
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
      150,
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
      190,
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
      190,
      40,
      22
     ],
     "text": "arm",
     "outlettype": [
      ""
     ]
    }
   },
   {
    "box": {
     "id": "obj-5",
     "maxclass": "message",
     "numinlets": 2,
     "numoutlets": 1,
     "patching_rect": [
      160,
      190,
      55,
      22
     ],
     "text": "disarm",
     "outlettype": [
      ""
     ]
    }
   },
   {
    "box": {
     "id": "obj-6",
     "maxclass": "message",
     "numinlets": 2,
     "numoutlets": 1,
     "patching_rect": [
      225,
      190,
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
     "id": "obj-7",
     "maxclass": "ezdac~",
     "numinlets": 2,
     "numoutlets": 0,
     "patching_rect": [
      380,
      150,
      45,
      45
     ]
    }
   },
   {
    "box": {
     "id": "obj-8",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      20,
      240,
      100,
      22
     ],
     "text": "route rec buf",
     "outlettype": [
      "",
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
     "numoutlets": 18,
     "patching_rect": [
      140,
      240,
      1000,
      22
     ],
     "text": "route cycle_440_ch0 dcblock_step_ch0 delay_echo_ch0 delay_echo_ch1 delay_echo_ch2 history_counter_ch0 history_counter_ch1 phasor_incr_order_ch0 range_inverted_bounds_ch0 range_inverted_bounds_ch1 range_inverted_bounds_ch2 sah_latch_ch0 sah_latch_ch1 slide_step_ch0 triangle_duty_ch0 triangle_duty_ch1 triangle_duty_ch2",
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
      300,
      220,
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
       600.0,
       450.0
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
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          360.0,
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
      335,
      70,
      22
     ],
     "text": "record~ cycle_440_ch0",
     "outlettype": [
      "signal"
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
      20,
      370,
      70,
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
     "id": "obj-13",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      280,
      300,
      220,
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
       600.0,
       450.0
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
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          360.0,
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
     "id": "obj-14",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      280,
      335,
      70,
      22
     ],
     "text": "record~ dcblock_step_ch0",
     "outlettype": [
      "signal"
     ]
    }
   },
   {
    "box": {
     "id": "obj-15",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      280,
      370,
      70,
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
      540,
      300,
      220,
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
       600.0,
       450.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// delay_echo.genexpr\n// Impulse at sample 0 fed into a 64-sample delay line with three taps.\n// Tests delay_write + delay_read with linear interpolation.\n// out1 = tap at 1 sample (1-sample delayed impulse)\n// out2 = tap at 4 samples (4-sample delayed impulse)\n// out3 = tap at 16 samples (16-sample delayed impulse)\n//\n// Impulse generated via history counter:\n//   h[n] = h[n-1] + 1, imp[n] = (h[n] == 0) \u2192 fires at n=0 only.\nh = history(h + 1);\nimp = eq(h, 0);\nDelay d(64);\nd.write(imp);\nout1 = d.read(1);\nout2 = d.read(4);\nout3 = d.read(16);\n",
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
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          360.0,
          60.0,
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
          110.0,
          360.0,
          60.0,
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
          190.0,
          360.0,
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
      540,
      335,
      70,
      22
     ],
     "text": "record~ delay_echo_ch0",
     "outlettype": [
      "signal"
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
      540,
      370,
      70,
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
     "id": "obj-19",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      615,
      335,
      70,
      22
     ],
     "text": "record~ delay_echo_ch1",
     "outlettype": [
      "signal"
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
      615,
      370,
      70,
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
     "id": "obj-21",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      690,
      335,
      70,
      22
     ],
     "text": "record~ delay_echo_ch2",
     "outlettype": [
      "signal"
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
      690,
      370,
      70,
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
     "id": "obj-23",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 2,
     "patching_rect": [
      800,
      300,
      220,
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
       600.0,
       450.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// history_counter.genexpr\n// Impulse-at-sample-zero via history counter.\n// h[n] = h[n-1] + 1, h[0] = 0 (zero-initialized history).\n// imp = (h == 0) \u2192 1.0 at sample 0 only, 0.0 thereafter.\n// out1 = impulse train (single impulse at origin).\n// out2 = counter value (monotonic integer sequence 0, 1, 2, ...).\nh = history(h + 1);\nimp = eq(h, 0);\nout1 = imp;\nout2 = h;\n",
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
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          360.0,
          60.0,
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
          110.0,
          360.0,
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
     "id": "obj-24",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      800,
      335,
      70,
      22
     ],
     "text": "record~ history_counter_ch0",
     "outlettype": [
      "signal"
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
      800,
      370,
      70,
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
     "id": "obj-26",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      875,
      335,
      70,
      22
     ],
     "text": "record~ history_counter_ch1",
     "outlettype": [
      "signal"
     ]
    }
   },
   {
    "box": {
     "id": "obj-27",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      875,
      370,
      70,
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
     "id": "obj-28",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      430,
      220,
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
       600.0,
       450.0
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
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          360.0,
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
     "id": "obj-29",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      465,
      70,
      22
     ],
     "text": "record~ phasor_incr_order_ch0",
     "outlettype": [
      "signal"
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
      500,
      70,
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
     "id": "obj-31",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      280,
      430,
      220,
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
       600.0,
       450.0
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
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          360.0,
          60.0,
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
          110.0,
          360.0,
          60.0,
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
          190.0,
          360.0,
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
     "id": "obj-32",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      280,
      465,
      70,
      22
     ],
     "text": "record~ range_inverted_bounds_ch0",
     "outlettype": [
      "signal"
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
      280,
      500,
      70,
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
     "id": "obj-34",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      355,
      465,
      70,
      22
     ],
     "text": "record~ range_inverted_bounds_ch1",
     "outlettype": [
      "signal"
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
      355,
      500,
      70,
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
     "id": "obj-36",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      430,
      465,
      70,
      22
     ],
     "text": "record~ range_inverted_bounds_ch2",
     "outlettype": [
      "signal"
     ]
    }
   },
   {
    "box": {
     "id": "obj-37",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      430,
      500,
      70,
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
     "id": "obj-38",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 2,
     "patching_rect": [
      540,
      430,
      220,
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
       600.0,
       450.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// sah_latch.genexpr\n// Sample-and-hold and latch driven by history counter.\n// h = {0, 1, 2, 3, ..., 4095}\n//\n// sah: samples h when h crosses 2.5 (trigger at h=3).\n//   output: held=0 until sample 3, then 3 forever.\n//\n// latch: passes h when h is non-zero.\n//   output: h=0\u21920 (held), h=1\u21921, h=2\u21922, ...\nh = history(h + 1);\nout1 = sah(h, h, 2.5);\nout2 = latch(h, h);\n",
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
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          360.0,
          60.0,
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
          110.0,
          360.0,
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
     "id": "obj-39",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      540,
      465,
      70,
      22
     ],
     "text": "record~ sah_latch_ch0",
     "outlettype": [
      "signal"
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
      540,
      500,
      70,
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
     "id": "obj-41",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      615,
      465,
      70,
      22
     ],
     "text": "record~ sah_latch_ch1",
     "outlettype": [
      "signal"
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
      615,
      500,
      70,
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
     "id": "obj-43",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      800,
      430,
      220,
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
       600.0,
       450.0
      ],
      "boxes": [
       {
        "box": {
         "id": "cb-1",
         "maxclass": "codebox",
         "code": "// slide_step.genexpr\n// Step response of slide (logarithmic smoother).\n// Step from 0 to 1 at sample 1 (sample 0 is 0).\n// Slide time constants: up=4, down=4 samples.\n// out1 = slewed step (asymptotic approach to 1.0).\nh = history(h + 1);\nstep = switch(gt(h, 0), 1, 0);\nout1 = slide(step, 4, 4);\n",
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
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          360.0,
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
     "id": "obj-44",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      800,
      465,
      70,
      22
     ],
     "text": "record~ slide_step_ch0",
     "outlettype": [
      "signal"
     ]
    }
   },
   {
    "box": {
     "id": "obj-45",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      800,
      500,
      70,
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
     "id": "obj-46",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 3,
     "patching_rect": [
      20,
      560,
      220,
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
       600.0,
       450.0
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
         "id": "go-1",
         "maxclass": "newobj",
         "text": "out 1",
         "numinlets": 1,
         "numoutlets": 0,
         "patching_rect": [
          30.0,
          360.0,
          60.0,
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
          110.0,
          360.0,
          60.0,
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
          190.0,
          360.0,
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
     "id": "obj-47",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      20,
      595,
      70,
      22
     ],
     "text": "record~ triangle_duty_ch0",
     "outlettype": [
      "signal"
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
      20,
      630,
      70,
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
     "id": "obj-49",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      95,
      595,
      70,
      22
     ],
     "text": "record~ triangle_duty_ch1",
     "outlettype": [
      "signal"
     ]
    }
   },
   {
    "box": {
     "id": "obj-50",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      95,
      630,
      70,
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
     "id": "obj-51",
     "maxclass": "newobj",
     "numinlets": 1,
     "numoutlets": 1,
     "patching_rect": [
      170,
      595,
      70,
      22
     ],
     "text": "record~ triangle_duty_ch2",
     "outlettype": [
      "signal"
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
      170,
      630,
      70,
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
      "obj-5",
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
      1
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
      "obj-10",
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
      "obj-8",
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
      0
     ],
     "destination": [
      "obj-12",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-13",
      0
     ],
     "destination": [
      "obj-14",
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
      "obj-14",
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
      "obj-15",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-16",
      0
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
      "obj-8",
      0
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
      2
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
      "obj-16",
      1
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
      "obj-8",
      0
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
      3
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
      "obj-16",
      2
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
      "obj-8",
      0
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
      4
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
      "obj-23",
      0
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
      "obj-8",
      0
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
      5
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
      "obj-23",
      1
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
      "obj-8",
      0
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
      6
     ],
     "destination": [
      "obj-27",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-28",
      0
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
      "obj-8",
      0
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
      7
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
      "obj-31",
      0
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
      "obj-8",
      0
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
      8
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
      "obj-31",
      1
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
      "obj-8",
      0
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
      "obj-9",
      9
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
      "obj-31",
      2
     ],
     "destination": [
      "obj-36",
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
      "obj-36",
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
      "obj-37",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-38",
      0
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
      "obj-8",
      0
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
      11
     ],
     "destination": [
      "obj-40",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-38",
      1
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
      "obj-8",
      0
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
      12
     ],
     "destination": [
      "obj-42",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-43",
      0
     ],
     "destination": [
      "obj-44",
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
      "obj-44",
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
      "obj-45",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-46",
      0
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
      "obj-8",
      0
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
      14
     ],
     "destination": [
      "obj-48",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-46",
      1
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
      "obj-8",
      0
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
      15
     ],
     "destination": [
      "obj-50",
      0
     ]
    }
   },
   {
    "patchline": {
     "source": [
      "obj-46",
      2
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
      "obj-8",
      0
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
      16
     ],
     "destination": [
      "obj-52",
      0
     ]
    }
   }
  ],
  "dependency_cache": [],
  "autosave": 0
 }
}

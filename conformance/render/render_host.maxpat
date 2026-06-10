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
    "rect": [0, 0, 600, 400],
    "bgcolor": [0.129412, 0.129412, 0.129412, 1.0],
    "bglocked": 1,
    "autosave": 1,
    "boxes": [
      {
        "box": {
          "id": "obj-1",
          "maxclass": "newobj",
          "numinlets": 1,
          "numoutlets": 1,
          "patching_rect": [25, 40, 150, 23],
          "fontsize": 12,
          "text": "gen~ @gen patches/phasor_incr_order.genexpr",
          "varname": "genpatcher",
          "outlettype": ["signal"]
        }
      },
      {
        "box": {
          "id": "obj-2",
          "maxclass": "newobj",
          "numinlets": 3,
          "numoutlets": 1,
          "patching_rect": [25, 100, 180, 23],
          "fontsize": 12,
          "text": "gain~ 0.0",
          "outlettype": ["signal"]
        }
      },
      {
        "box": {
          "id": "obj-3",
          "maxclass": "newobj",
          "numinlets": 3,
          "numoutlets": 1,
          "patching_rect": [25, 160, 150, 23],
          "fontsize": 12,
          "text": "record~ capture 1",
          "varname": "recorder",
          "outlettype": ["signal"]
        }
      },
      {
        "box": {
          "id": "obj-4",
          "maxclass": "newobj",
          "numinlets": 2,
          "numoutlets": 1,
          "patching_rect": [25, 220, 180, 23],
          "fontsize": 12,
          "text": "buffer~ capture 4096",
          "varname": "capturebuf"
        }
      },
      {
        "box": {
          "id": "obj-5",
          "maxclass": "ezdac~",
          "numinlets": 2,
          "numoutlets": 0,
          "patching_rect": [25, 280, 25, 25],
          "signal": [1, 0]
        }
      },
      {
        "box": {
          "id": "obj-6",
          "maxclass": "newobj",
          "numinlets": 1,
          "numoutlets": 2,
          "patching_rect": [250, 100, 200, 23],
          "fontsize": 12,
          "text": "node.script render_runner.js",
          "varname": "nodeworker"
        }
      },
      {
        "box": {
          "id": "obj-7",
          "maxclass": "message",
          "numinlets": 2,
          "numoutlets": 1,
          "patching_rect": [250, 40, 250, 23],
          "fontsize": 12,
          "text": "; max runtime start_record = 1",
          "outlettype": [""]
        }
      },
      {
        "box": {
          "id": "obj-8",
          "maxclass": "comment",
          "numinlets": 1,
          "numoutlets": 0,
          "patching_rect": [25, 320, 550, 50],
          "fontsize": 12,
          "text": "GenExpr Conformance Render Host\nOpen in Max 9. With audio enabled, node.script drives rendering.\nGennum patches are auto-loaded; each renders 4096 samples to conformance/golden/<stem>.ch<N>.wav"
        }
      }
    ],
    "lines": [
      {
        "patchline": {
          "source": ["obj-1", 0],
          "destination": ["obj-2", 0],
          "midpoints": []
        }
      },
      {
        "patchline": {
          "source": ["obj-2", 0],
          "destination": ["obj-3", 0],
          "midpoints": []
        }
      },
      {
        "patchline": {
          "source": ["obj-2", 0],
          "destination": ["obj-5", 0],
          "midpoints": []
        }
      }
    ]
  }
}

# L4D addon for benilla

This is the fork's HUD, copied from `LyraCore/packages/l4d-northshire/client/L4D`. benilla only
loads addons from its state folder, so install it there:

```sh
mkdir -p benilla-config/AddOns
cp -r extras/addons/L4D benilla-config/AddOns/
```

`benilla-config` is created next to the launcher on first run. In a dev build that is the
directory cargo ran from. Restart the client after copying; addon lists are read at interface load.

The addon draws the health/bleed HUD, crosshair and director bar from server bridge messages. It
does not patch FrameXML.

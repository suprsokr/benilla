-- L4D Northshire: a compact health / bleed / director HUD driven by the server's bridge.
-- The server sends CHAT_MSG_ADDON lines with prefix STC and a v1 envelope:
--   v1|l4d.state|0|1/1|hp|max|wounded|downed|down_ms|knockdowns|intensity|phase|wave|profile|alive|smoked|blinded
--   v1|l4d.hit|0|1/1|damage|profile|crit|extras|splash
--   v1|l4d.wave|0|1/1|count|front|left|right|behind
--   v1|l4d.knockdown|0|1/1|count|length_ms
--   v1|l4d.kill|0|1/1|entry|gain|applied
--   v1|l4d.special|0|1/1|hunter|smoker|tank
-- The stock unit frame stays available; this HUD is the at-a-glance one.

local WHITE = "Interface\\Buttons\\WHITE8X8"
local LOW_HEALTH = "Interface\\FullScreenTextures\\LowHealth"

local frame = CreateFrame("Frame", "L4DHUD", UIParent)
frame:Hide()

local vignette = UIParent:CreateTexture("L4DHUDVignette", "BACKGROUND")
vignette:SetTexture(LOW_HEALTH)
vignette:SetAllPoints(UIParent)
vignette:SetAlpha(0)
vignette:SetBlendMode("ADD")

local cross = CreateFrame("Frame", "L4DHUDCrosshair", UIParent)
cross:SetWidth(26)
cross:SetHeight(26)
cross:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
cross:Hide()

local arms = {}
for i = 1, 4 do
  local t = cross:CreateTexture(nil, "OVERLAY")
  t:SetTexture(WHITE)
  t:SetVertexColor(1, 1, 1, 0.85)
  arms[i] = t
end
arms[1]:SetWidth(2)
arms[1]:SetHeight(8)
arms[1]:SetPoint("TOP", cross, "TOP", 0, 0)
arms[2]:SetWidth(2)
arms[2]:SetHeight(8)
arms[2]:SetPoint("BOTTOM", cross, "BOTTOM", 0, 0)
arms[3]:SetWidth(8)
arms[3]:SetHeight(2)
arms[3]:SetPoint("LEFT", cross, "LEFT", 0, 0)
arms[4]:SetWidth(8)
arms[4]:SetHeight(2)
arms[4]:SetPoint("RIGHT", cross, "RIGHT", 0, 0)

local killText = UIParent:CreateFontString("L4DHUDKill", "OVERLAY", "GameFontNormalLarge")
killText:SetPoint("CENTER", UIParent, "CENTER", 0, 70)
killText:SetTextColor(1, 0.85, 0.35)
killText:SetText("")
killText:Hide()

local panel = CreateFrame("Frame", nil, frame)
panel:SetWidth(250)
panel:SetHeight(54)
panel:SetPoint("BOTTOM", UIParent, "BOTTOM", 0, 158)

local backdrop = panel:CreateTexture(nil, "BACKGROUND")
backdrop:SetTexture(WHITE)
backdrop:SetVertexColor(0, 0, 0, 0.55)
backdrop:SetAllPoints(panel)

local healthBar = CreateFrame("StatusBar", "L4DHUDHealth", panel)
healthBar:SetWidth(250)
healthBar:SetHeight(18)
healthBar:SetPoint("BOTTOM", panel, "BOTTOM", 0, 22)
healthBar:SetStatusBarTexture(WHITE)
healthBar:SetStatusBarColor(0.2, 0.8, 0.2)
healthBar:SetMinMaxValues(0, 100)

local healthText = healthBar:CreateFontString(nil, "OVERLAY", "GameFontNormal")
healthText:SetPoint("CENTER", healthBar, "CENTER", 0, 0)
healthText:SetText("100 / 100")

local stateText = panel:CreateFontString(nil, "OVERLAY", "GameFontNormalSmall")
stateText:SetPoint("BOTTOM", healthBar, "TOP", 0, 3)
stateText:SetText("")

local directorBar = CreateFrame("StatusBar", "L4DHUDDirector", panel)
directorBar:SetWidth(250)
directorBar:SetHeight(4)
directorBar:SetPoint("BOTTOM", panel, "BOTTOM", 0, 12)
directorBar:SetStatusBarTexture(WHITE)
directorBar:SetStatusBarColor(0.85, 0.2, 0.2)
directorBar:SetMinMaxValues(0, 100)

local infoText = panel:CreateFontString(nil, "OVERLAY", "GameFontNormalSmall")
infoText:SetPoint("BOTTOM", panel, "BOTTOM", 0, 0)
infoText:SetText("")

local WARNING_Y = 120
local WARNING_SLIDE = 220

local warning = CreateFrame("Frame", nil, UIParent)
warning:SetWidth(500)
warning:SetHeight(40)
warning:SetPoint("CENTER", UIParent, "CENTER", 0, WARNING_Y)

local warningText = warning:CreateFontString(nil, "OVERLAY", "GameFontNormalLarge")
warningText:SetPoint("CENTER", warning, "CENTER", 0, 0)
warningText:SetTextColor(1, 0.15, 0.1)
warningText:SetText("")
warning:Hide()

local function place_warning(dx)
  warning:ClearAllPoints()
  warning:SetPoint("CENTER", UIParent, "CENTER", dx, WARNING_Y)
end

local state = {
  active = false,
  wounded = false,
  downed = false,
  down_ms = 0,
  knockdowns = 0,
  intensity = 0,
  phase = 0,
  wave = 0,
  profile = "fists",
  alive = 0,
  smoked = false,
  blinded = false,
  hit_flash = 0,
  combo = 0,
  combo_until = 0,
  kill_until = 0,
  wave_until = 0,
  cleave_until = 0,
}

local function split(text)
  local out = {}
  for field in string.gmatch(text .. "|", "([^|]*)|") do
    out[#out + 1] = field
  end
  return out
end

local function num(value, fallback)
  return tonumber(value) or fallback or 0
end

local function update_health()
  local health = UnitHealth("player") or 0
  local max_health = UnitHealthMax("player") or 1
  if max_health < 1 then
    max_health = 1
  end
  healthBar:SetMinMaxValues(0, max_health)
  healthBar:SetValue(health)
  healthText:SetText(string.format("%d / %d", health, max_health))

  local fraction = health / max_health
  if fraction <= 0.4 then
    healthBar:SetStatusBarColor(0.85, 0.15, 0.1)
  elseif fraction <= 0.55 then
    healthBar:SetStatusBarColor(0.85, 0.75, 0.1)
  else
    healthBar:SetStatusBarColor(0.2, 0.8, 0.2)
  end

  if state.downed then
    vignette:SetAlpha(0.75)
    vignette:SetVertexColor(1, 0.1, 0.05)
  elseif fraction < 0.4 then
    local depth = (0.4 - fraction) / 0.4
    vignette:SetAlpha(0.15 + depth * 0.6)
    vignette:SetVertexColor(1, 0.15, 0.1)
  elseif state.wounded then
    vignette:SetAlpha(0.12)
    vignette:SetVertexColor(1, 0.3, 0.1)
  else
    vignette:SetAlpha(0)
  end

  local label = ""
  if state.downed then
    label = string.format("DOWN  %ds   (%d/3)", math.ceil(state.down_ms / 1000), state.knockdowns)
    stateText:SetTextColor(1, 0.2, 0.15)
  elseif state.wounded then
    label = "WOUNDED"
    stateText:SetTextColor(1, 0.55, 0.2)
  else
    stateText:SetTextColor(0.8, 0.9, 0.8)
  end
  stateText:SetText(label)

  local phase_names = { "calm", "building", "peak", "HORDE" }
  infoText:SetText(string.format(
    "%s   threat %d%% (%s)   wave %d   infected %d",
    state.profile,
    state.intensity,
    phase_names[state.phase + 1] or "calm",
    state.wave,
    state.alive
  ))
  directorBar:SetValue(state.intensity)
  if state.phase >= 3 then
    directorBar:SetStatusBarColor(1, 0.1, 0.1)
  elseif state.phase == 2 then
    directorBar:SetStatusBarColor(0.9, 0.45, 0.1)
  else
    directorBar:SetStatusBarColor(0.85, 0.2, 0.2)
  end
end

local function apply_state(payload)
  local f = split(payload)
  state.wounded = num(f[3]) == 1
  state.downed = num(f[4]) == 1
  state.down_ms = num(f[5])
  state.knockdowns = num(f[6])
  state.intensity = num(f[7])
  state.phase = num(f[8])
  state.wave = num(f[9])
  state.profile = f[10] or "fists"
  state.alive = num(f[11])
  state.smoked = num(f[12]) == 1
  state.blinded = num(f[13]) == 1
  if state.smoked then
    warningText:SetText("SMOKER  -  MOVE")
    warning:Show()
  elseif state.blinded then
    warningText:SetText("BILE  -  HERE THEY COME")
    warning:Show()
  elseif not state.downed and GetTime() >= (state.wave_until or 0) then
    warning:Hide()
  end
  update_health()
end

frame:SetScript("OnEvent", function(self, event, ...)
  if event == "ADDON_LOADED" then
    frame:RegisterEvent("PLAYER_ENTERING_WORLD")
    frame:RegisterEvent("ZONE_CHANGED_NEW_AREA")
    frame:RegisterEvent("PLAYER_DEAD")
    frame:RegisterEvent("PLAYER_ALIVE")
    frame:RegisterEvent("CHAT_MSG_ADDON")
    frame:RegisterEvent("PLAYER_REGEN_DISABLED")
  elseif event == "CHAT_MSG_ADDON" then
    local prefix, message = ...
    if prefix == "STC" and message then
      local cmd, payload = string.match(message, "^v1|([^|]+)|%d+|1/1|(.*)$")
      if cmd == "l4d.state" then
        apply_state(payload)
      elseif cmd == "l4d.hit" then
        state.hit_flash = 0.18
        local f = split(payload)
        local extras = num(f[4])
        local now = GetTime()
        if extras > 0 and now >= state.combo_until then
          state.cleave_until = now + 0.12
          killText:SetText(string.format("CLEAVE %d", extras))
          killText:Show()
          cross:SetScale(1.2)
        end
      elseif cmd == "l4d.wave" then
        if state.downed then
          return
        end
        local f = split(payload)
        local count = num(f[1])
        local left = num(f[3])
        local right = num(f[4])
        local behind = num(f[5])
        local dx = 0
        if left > right then
          dx = -WARNING_SLIDE
        elseif right > left then
          dx = WARNING_SLIDE
        end
        place_warning(dx)
        warningText:SetText(string.format("HORDE %d   %d BEHIND", count, behind))
        state.wave_until = GetTime() + 2
        warning:Show()
      elseif cmd == "l4d.kill" then
        local f = split(payload)
        local applied = num(f[3])
        local now = GetTime()
        state.combo = (now < state.combo_until) and state.combo + 1 or 1
        state.combo_until = now + 2
        state.kill_until = now + 0.12
        if applied > 0 then
          killText:SetText(string.format("+%d  x%d", applied, state.combo))
        else
          killText:SetText(string.format("x%d", state.combo))
        end
        killText:Show()
        cross:SetScale(1.35)
      elseif cmd == "l4d.knockdown" then
        local f = split(payload)
        state.knockdowns = num(f[1])
        state.down_ms = num(f[2])
        state.downed = true
        warningText:SetText("YOU ARE DOWN")
        warning:Show()
      elseif cmd == "l4d.special" then
        if payload == "tank" then
          warningText:SetText("TANK")
          warning:Show()
        end
      end
    end
  elseif event == "PLAYER_ENTERING_WORLD" or event == "ZONE_CHANGED_NEW_AREA" then
    local zone = GetZoneText() or ""
    local sub = GetSubZoneText() or ""
    state.active = string.find(zone, "Elwynn") ~= nil
      or string.find(sub, "Northshire") ~= nil
      or string.find(zone, "Northshire") ~= nil
    if state.active then
      frame:Show()
      cross:Show()
      update_health()
    else
      frame:Hide()
      cross:Hide()
      place_warning(0)
      warning:Hide()
      vignette:SetAlpha(0)
    end
  elseif event == "PLAYER_DEAD" then
    state.downed = false
    vignette:SetAlpha(0.7)
    vignette:SetVertexColor(0.3, 0.3, 0.3)
  elseif event == "PLAYER_ALIVE" then
    vignette:SetAlpha(0)
  end
end)

frame:SetScript("OnUpdate", function(self, elapsed)
  update_health()
  if state.hit_flash > 0 then
    state.hit_flash = state.hit_flash - elapsed
    local alpha = state.hit_flash > 0 and 1 or 0.85
    for i = 1, 4 do
      arms[i]:SetVertexColor(1, 0.6, 0.4, alpha)
    end
  else
    for i = 1, 4 do
      arms[i]:SetVertexColor(1, 1, 1, 0.85)
    end
  end
  local now = GetTime()
  if state.kill_until > 0 and now >= state.kill_until then
    state.kill_until = 0
    cross:SetScale(1)
  end
  if state.cleave_until > 0 and now >= state.cleave_until then
    state.cleave_until = 0
    cross:SetScale(1)
  end
  if state.wave_until > 0 and now >= state.wave_until then
    state.wave_until = 0
    place_warning(0)
    if not state.downed and not state.smoked and not state.blinded then
      warning:Hide()
    end
  end
  if killText:IsShown() and now >= state.combo_until and now >= state.cleave_until then
    killText:Hide()
    state.combo = 0
  end
end)

function hideout_entrance(parent)
  game:cancel_blocking_anims()
  game:scroll_view(34, 26)
  local thug = game:entity_with_id("thug04")
  game:start_conversation("thugs_hideout_enter", thug)
end

function gethruk_talk(parent)
  game:cancel_blocking_anims()
  game:scroll_view(57, 57)
  local gethruk = game:entity_with_id("gethruk_boss")
  game:start_conversation("gethruk_boss", gethruk)
end

function set_thugs_hostile()
  game:player():set_flag("fought_gethruk_thugs")

  local entities = game:entities_with_ids({"thug04", "thug05", "thug06", "thug07", "thug08"})
  set_hostile(entities)
end

function thugs_pay_off()
  game:add_party_xp(50)
end

function set_boss_hostile()
  local entities = game:entities_with_ids({"thug09", "thug10", "thug11", "thug12", "gethruk_boss"})
  set_hostile(entities)
  
  entities = game:entities_with_ids({"thug04", "thug05", "thug06", "thug07", "thug08"})
  set_hostile(entities)
end

function set_hostile(entities)
  for i = 1, #entities do
    entities[i]:set_faction("Hostile")
  end
end

function gethruk_cleared(parent)
  game:set_quest_entry_state("the_thug", "gethruk_dead", "Visible")
  game:player():set_flag("gethruk_cleared")
end

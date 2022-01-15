#[crud_table(table_name:"player" | table_columns:"id,profileName,loginToken,exp,selectedFormation,selectedArenaFormation,mainCharacter,mainCharacterExp,arenaScore,highestArenaRank,highestArenaRankCurrentSeason,clanId,clanRole,createdAt,updatedAt")]
#[derive(Clone, Debug)]
pub struct Player {
  pub id: Option<i64>,
  pub profile_name: Option<String>,
  pub login_token: Option<String>,
  pub exp: Option<i32>,
  pub selected_formation: Option<String>,
  pub selected_arena_formation: Option<String>,
  pub main_character: Option<String>,
  pub main_character_exp: Option<i32>,
  pub arena_score: Option<i32>,
  pub highest_arena_rank: Option<i32>,
  pub highest_arena_rank_current_season: Option<i32>,
  pub clan_id: Option<i64>,
  pub clan_role: Option<i8>,
  pub created_at: Option<rbatis::DateTimeNative>,
  pub updated_at: Option<rbatis::DateTimeNative>,
}
export type MoodCount = {
  mood_1: number
  mood_2: number
  mood_3: number
  mood_4: number
  mood_5: number
}

export type MoodStats = {
  entry_count: number
  average_mood: number
  median_mood: number
}

export type MoodStatsWithCount = MoodStats & {
  mood_entry_count: MoodCount
}

export type TagMoodStats = {
  tag_id: string
  entry_count: number
  average_mood: number
  median_mood: number
}

export type TagMoodStatsWithCount = TagMoodStats & {
  mood_entry_count: MoodCount
}

export type TagStats = TagMoodStats[]
export type TagStatsWithCount = TagMoodStatsWithCount[]

export type WeekdayStats = {
  monday: MoodStats
  tuesday: MoodStats
  wednesday: MoodStats
  thursday: MoodStats
  friday: MoodStats
  saturday: MoodStats
  sunday: MoodStats
}

export type WeekdayStatsWithCount = {
  monday: MoodStatsWithCount
  tuesday: MoodStatsWithCount
  wednesday: MoodStatsWithCount
  thursday: MoodStatsWithCount
  friday: MoodStatsWithCount
  saturday: MoodStatsWithCount
  sunday: MoodStatsWithCount
}

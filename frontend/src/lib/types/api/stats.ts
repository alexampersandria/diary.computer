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
  mood_entry_count: MoodCount
}

export type TagMoodStats = {
  tag_id: string
  entry_count: number
  average_mood: number
  median_mood: number
  mood_entry_count: MoodCount
}

export type TagStats = TagMoodStats[]

export type WeekdayStats = {
  monday: MoodStats
  tuesday: MoodStats
  wednesday: MoodStats
  thursday: MoodStats
  friday: MoodStats
  saturday: MoodStats
  sunday: MoodStats
}

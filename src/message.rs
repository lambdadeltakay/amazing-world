use binrw::BinReaderExt;
use bitvec::field::BitField;
use bitvec::prelude::{Lsb0, Msb0};
use bitvec::view::BitView;
use chrono::{DateTime, Utc};
use nom::combinator::cond;
use nom::error::Error;
use nom::sequence::{tuple, Tuple};
use nom::{bits, bytes};
use nom::{IResult, Parser};
use num_enum::TryFromPrimitive;
use std::io::Cursor;
use std::mem::size_of;
use std::ops::Deref;

#[derive(PartialEq, Eq, Hash, Debug, TryFromPrimitive)]
#[repr(i64)]
pub enum ClientMessage {
    AddObject = 1,
    MoveObject = 2,
    RemoveObject = 3,
    ChangeObject = 4,
    ServerChangeObject = 5,
    MovePlayer = 6,
    RemovePlayer = 7,
    Chat = 8,
    StartEvent = 9,
    StopEvent = 10,
    Emote = 11,
    ChangeWeight = 12,
    Pause = 13,
    Resume = 14,
    Notification = 16,
    ChangeServer = 17,
    SendNotify = 18,
    IntList = 19,
    AddPlayer = 20,
    UpdateNpcs = 21,
    StopNpc = 22,
    Minimap = 23,
    PosRecap = 24,
    Action = 25,
    Evict = 26,
    OnlineStatus = 27,
    ChangeObjectState = 28,
}

#[derive(PartialEq, Eq, Hash, Debug, TryFromPrimitive)]
#[repr(i64)]
pub enum UserMessage {
    GetAvatars = 1,
    ClaimGift = 2,
    RegisterPlayer = 3,
    GetPlayerMazes = 4,
    StartMazeEdit = 5,
    GetPlayerMaze = 6,
    GetCommunityMaze = 7,
    EndMazeEdit = 8,
    DeleteMaze = 9,
    GetDynamicSurprise = 10,
    GetZoneMazes = 11,
    GetCommunityMazes = 12,
    GetQuestMazes = 13,
    PublishMaze = 14,
    Login = 15,
    GetPlayerNpcs = 16,
    CheckUsername = 17,
    RegisterFcssCode = 18,
    Logout = 19,
    GetInventoryObjects = 20,
    GetBuildObjects = 21,
    GetCollectionObjects = 22,
    UpdateAvatarName = 23,
    SearchVillages = 24,
    MoveVillage = 25,
    Relogin = 26,
    InitLocation = 27,
    GetLangLocale = 28,
    EnterBuilding = 29,
    BuyItem = 30,
    GetShoppingItems = 31,
    ListShoppingCategories = 32,
    GetFriendList = 33,
    RemoveFriend = 34,
    ManageFriendRequest = 35,
    GetBlockedPlayers = 36,
    ManageBlockPlayer = 37,
    AddFriend = 38,
    UnpublishMaze = 39,
    GetRandomWorldName = 40,
    GetOnlineStatus = 41,
    ManageTestPlayer = 42,
    UpdateQuestItem = 43,
    PlaceMazeItem = 44,
    RemoveMazeItem = 45,
    GetPlayerMazeThumbnails = 46,
    GetPlayerMazeThumbnail = 47,
    UpdatePlayerMazeThumbnail = 48,
    GetCommunityMazeThumbnails = 49,
    AcceptQuest = 50,
    CompleteQuest = 51,
    AddQuestItem = 52,
    EndGame = 53,
    GetPlayerDynamicSurprises = 54,
    ClaimDynamicSurprise = 55,
    GetNotifications = 56,
    GetNotificationByPlayerNotificationId = 57,
    GetNotificationByPlayerId = 58,
    UpdateNotification = 59,
    GetPlayerMazeRating = 60,
    GetPlayerMazeRatings = 61,
    GetSystemMazeRating = 62,
    RatePlayerMaze = 63,
    GetNotificationOptions = 64,
    UpdateNotificationOption = 65,
    ClearNotifications = 66,
    GetHomeMaze = 67,
    UpdateHomeMaze = 68,
    GetNotificationCategories = 69,
    GetNotificationOptionByCategory = 70,
    AcknowledgeNotification = 71,
    GetNotificationOptionByPlayerNotificationOptionId = 72,
    GetNotificationCategory = 73,
    ClearNotificationsByPlayerId = 74,
    SetNotificationOptions = 75,
    GetCmsCollectionItems = 76,
    AddCollectionItem = 77,
    DropCollectionItem = 78,
    ResetCollection = 79,
    ApproveMazePublishing = 80,
    UpdateFriendComment = 81,
    GetActiveFriendList = 82,
    GetFriendRequests = 83,
    GetFriendByFriendPlayerId = 84,
    FindFriendByPlayerFriendId = 85,
    GetQuestTypes = 86,
    DeleteFriendByPlayerFriendId = 87,
    SendPrivateChatGroupInvite = 88,
    SendMessage = 89,
    GetCmsItemcategoryIds = 90,
    GetCmsItemcategoryById = 91,
    GetPlayerMazePlay = 92,
    ConsumeInventoryItem = 93,
    MoveInventoryItem = 94,
    SwapInventoryItems = 95,
    RedeemFeatureCode = 96,
    GetMazeItems = 97,
    GetYardItems = 98,
    PlaceYardItem = 99,
    RemoveYardItem = 100,
    GetSystemMazePlay = 101,
    GetAvatarItems = 102,
    GetOutfitItems = 103,
    StartPrivateChatGroup = 104,
    AcceptPrivateChatGroupInvite = 105,
    LeavePrivateGroup = 106,
    GetChatChannelTypes = 107,
    StartMazePlay = 108,
    EndMazePlay = 109,
    DressAvatar = 110,
    GetOutfits = 111,
    StartQuest = 112,
    Heartbeat = 113,
    GetFriendAccountsByBirthday = 114,
    GetPrivateChatGroupMembers = 115,
    RejoinPrivateChatGroup = 116,
    AddPlayerInfraction = 117,
    GetPlayerChatHistory = 118,
    GetQuestItems = 119,
    StartGame = 120,
    GetMazeSets = 121,
    DeclinePrivateChatGroupInvite = 122,
    GetMazeSet = 123,
    FindPrivateChatGroupMember = 124,
    FindPrivateChatGroupId = 125,
    CompleteDailyActivity = 126,
    GetQuestById = 127,
    GetWeeklyAwardStatus = 128,
    GetPlayerWeeklyAwardHistory = 129,
    GetWeeklyAwardStatusByPlayerWeeklyAwardId = 130,
    GetSlots = 131,
    GetGames = 132,
    GetQuests = 133,
    GetDailyAwardStatus = 134,
    UpdatePlayerActiveAvatar = 135,
    GetSiteFrame = 136,
    GetCmsItemcategories = 137,
    NextCaptcha = 138,
    CheckCaptcha = 139,
    GetPlayerGifts = 140,
    UpdateHomeTheme = 141,
    GetHomeThemes = 142,
    GetPlayerHomeThemes = 143,
    RejectQuest = 144,
    GetPlayerReceivedGifts = 145,
    ManageGiftRequest = 146,
    GetAnnouncements = 147,
    GetAnnouncement = 148,
    GetAchievementById = 149,
    GetAchievements = 150,
    GetAdoptionObjectsByAvatar = 151,
    GetObjectIdsByAdoptionNumber = 152,
    FindGiftByPlayerGiftId = 153,
    GetZones = 154,
    ListStoreCategories = 155,
    ListStoreInventory = 156,
    ListStores = 157,
    PurchaseItems = 158,
    CompleteTutorial = 159,
    UpdateLanglocale = 160,
    UpdatePlayerName = 161,
    GetPlayerAchievements = 162,
    GetShards = 163,
    GetCraftableItems = 164,
    GetCraftableItemById = 165,
    PauseNpc = 166,
    ResumeNpc = 167,
    GetPlayerQuests = 168,
    NpcInteraction = 169,
    CraftItemByCraftableItemId = 170,
    GetNpcRelationships = 171,
    CraftItemByItems = 172,
    GetNpcs = 173,
    DressAvatarItems = 174,
    LockHome = 175,
    UndressAvatar = 176,
    GetMazePiecesByPlayerMazeId = 177,
    GetStoreThemes = 178,
    GetAssetsByOids = 179,
    GetCurrencies = 180,
    AddOutfitItems = 181,
    AddOutfit = 182,
    RemoveOutfitItems = 183,
    RemoveOutfit = 184,
    ReplaceOutfitItems = 185,
    SetCurrentOutfit = 186,
    GetFriendAvatars = 187,
    GetPlayerQuestsByQuestIds = 188,
    GetPlayerGamesByZone = 189,
    GetHomeInvitations = 190,
    ManageHomeInvitations = 191,
    FindPlayerByNickname = 192,
    GetPlayerAvatarsByBirthday = 193,
    SendQuestInvite = 194,
    GetHostedQuests = 195,
    GetInvitedQuests = 196,
    GetPlayersInQuest = 197,
    RemoveQuestInvite = 198,
    AcceptQuestInvite = 199,
    DeclineQuestInvite = 200,
    GetPlayerReceivedQuestInvite = 201,
    GetPlayerQuestInvite = 202,
    CleanupPlayerMazes = 204,
    GetPlayerStats = 205,
    ListStoreInventoryItems = 206,
    GetOtherPlayerDetails = 207,
    GetTestPlayers = 208,
    GetStatsType = 209,
    TestInventoryUpdates = 210,
    TestInventoryRemovals = 211,
    SetPlayerGiftStatus = 212,
    GetRequiredExperience = 213,
    GetQuestByParentId = 214,
    UpdateOnlineStatus = 215,
    ChangeStoreItemStock = 216,
    LogoutSession = 217,
    AbandonQuest = 218,
    GetAwardSets = 219,
    UpdateArm = 220,
    UpdateRuledObject = 221,
    UpdateRule = 222,
    AddAward = 223,
    UpdateAward = 224,
    UpdateStateObject = 225,
    GetStateObjects = 226,
    GetAwardById = 227,
    GetRuleById = 228,
    GetRuledObjectByHierarchy = 229,
    AddStateObject = 230,
    AddRule = 231,
    AddArm = 232,
    GetArmByAward = 233,
    GetArmByRule = 234,
    GetArmByStateObject = 235,
    GetAwardByArm = 236,
    GetRuleByArm = 237,
    GetStateObjectByArm = 238,
    ObjectInfoRows = 239,
    AssignRuledSetToRuledObject = 240,
    AddRuledObject = 241,
    GetRuleSets = 242,
    SaveObjectAttributes = 243,
    GetObjectTypes = 244,
    GetCmsNotifications = 245,
    JoinQuest = 246,
    GetAllOperators = 247,
    GetOnlineStatuses = 248,
    CstoolGetFullAccountInformation = 249,
    RemoveFriendCstool = 250,
    UpdateYard = 251,
    UpdateChatAvailability = 252,
    GetFriendListCstool = 253,
    CreateOrUpdateCurrencyCstool = 254,
    GetAllRules = 255,
    GetAllRuledObject = 256,
    GetAllAwards = 257,
    DeleteRule = 258,
    DeleteRuledObject = 259,
    DeleteAward = 260,
    GetFullRuleById = 261,
    GetRuledObjectById = 262,
    GetCurrencyCstool = 263,
    GetEnergyLevelCstool = 264,
    GetUserLevelCstool = 265,
    GetUserXpCstool = 266,
    GetUserNpcRelationshipLevel = 267,
    CreateOrUpdateEnergyLevelCstool = 268,
    CreateOrUpdateUserLevelCstool = 269,
    CreateOrUpdateUserXpCstool = 270,
    CreateOrUpdateUserNpcRelationshipLevel = 271,
    GetFullRuledObjectById = 272,
    GetFullAwardById = 273,
    DeleteAsam = 274,
    DeleteRsrm = 275,
    GetFormulaByType = 276,
    AddBugReport = 277,
    FindPlayerAccountCstool = 278,
    FindNpcsCstool = 279,
    FindAvatarBySkuCstool = 280,
    RegisterAvatarCstool = 281,
    GetAllCategories = 282,
    UnregisterTestPlayers = 283,
    GetAllAttributes = 284,
    UpdateStoreItemNotify = 285,
    GetAllAttributesByCategory = 286,
    AddObjectAttributes = 287,
    UpdateObjectAttributes = 288,
    GetArmById = 289,
    DeleteRuleFromRsrm = 290,
    DeleteAwardFromAsam = 291,
    GetAllArm = 292,
    AddRulesToRsrm = 293,
    AddAwardsToAsam = 294,
    GetNpcWithMostRelationship = 295,
    StateObjectArm = 296,
    GetStoreItems = 297,
    ManageCrispAction = 298,
    GetCrispActions = 299,
    SearchItems = 300,
    AddItemsToPlayer = 301,
    AddToPlayerStat = 302,
    ResendRegConfKey = 303,
    AcknowledgeConfKey = 304,
    GetCmsMissions = 305,
    GetRuleInfo = 306,
    GetPlayerQuestsCstool = 307,
    StartTaskCstool = 308,
    GetOtherPlayerNpcRelationships = 309,
    GetBuyBackStoreItems = 310,
    UpdateChatAvailabilityForPlayerCstool = 311,
    GetChatAvailabilityCstool = 312,
    GetStoreItemsForItems = 313,
    PurchaseQuest = 314,
    GetCmsEvents = 315,
    AddEvent = 316,
    RequestEvent = 317,
    GetPendingEvents = 318,
    AcceptEvent = 319,
    RejectEvent = 320,
    SavePlayerSettings = 321,
    LoadPlayerSettings = 322,
    GetRuledObjectByObjectId = 323,
    SuggestFriends = 324,
    SearchUsers = 325,
    CstoolUnlockPlayerAccount = 326,
    GetTiers = 327,
    GetDebugQuests = 328,
    GetNpcsWithQuestOffer = 329,
    CreatePlayerMission = 330,
    GetPlayerMissionDetail = 331,
    GetPlayerMissions = 332,
    DeleteItemFromPlayer = 333,
    DebugCreateQuest = 334,
    RegisterAvatarForRegistration = 335,
    UpdateAvatarNameForRegistration = 336,
    EnterStore = 337,
    ExitStore = 338,
    GetAllQuests = 339,
    GetItemsByCategory = 340,
    GetItemCategoriesCstool = 341,
    GetAllZones = 342,
    GetAllNpcs = 343,
    GetAllStoreItems = 344,
    UpdatePlayerEmail = 345,
    AddUserStoreItem = 346,
    AllocatePlayerVillage = 347,
    SearchUserStoreItems = 348,
    UpdateCrispAvailabilityCsTool = 349,
    GetAllVillages = 350,
    UpdateUserStoreItem = 351,
    RemoveUserStoreItem = 352,
    GetCrispAvailabilityCsTool = 353,
    GetPlayerCrispData = 354,
    UpdateChatBlockedByParentCsTool = 355,
    UpdateChatBlockedByParent = 356,
    GetPlayerWorldnameByGasid = 357,
    GetScsInvalidCodeStatus = 358,
    GetCrispActionsForNonloggedInsession = 359,
    GetPlayerContainerByPlayer = 360,
    GetPlayerContainerType = 361,
    GetWebContent = 362,
    ListItemByPlayer = 363,
    ListItemByContainer = 364,
    GetWebContentByPtag = 365,
    GetEula = 366,
    GetPlayerAccountByGasid = 367,
    GetPlayerIdByGasid = 368,
    ListVillagePlots = 369,
    SendVillageInvite = 370,
    AcceptVillageInvite = 371,
    RejectVillageInvite = 372,
    ReportAbuse = 373,
    GetAbuseReports = 374,
    GetAbuseReportsByPlayer = 375,
    UpdateAbuseReport = 376,
    PurchaseUserStoreItems = 377,
    DecorateUserStore = 378,
    RedeemUserStoreSales = 379,
    GetRuleInstanceInfo = 380,
    AcceptEula = 381,
    AddPlayerLike = 382,
    RemovePlayerLike = 383,
    ListTopLikes = 384,
    UndecorateUserStore = 385,
    UserStoreInfo = 386,
    AddAwardToAwardQueue = 387,
    RegistrationRecord = 388,
    GetObjectFromSku = 389,
    GetInfractionTypes = 390,
    GetAllCmsMissions = 391,
    CheckEmailAvailability = 392,
    AddBookmark = 393,
    RemoveBookmark = 394,
    ListBookmarks = 395,
    ListLimits = 396,
    ListVillageUsers = 397,
    GetDailyAwards = 398,
    ListPlayerLikes = 399,
    VillageInfo = 400,
    DecoratedUserStoreItems = 401,
    ListBookmarkObjectTypes = 402,
    LinkFacebookAccount = 403,
    UnlinkFacebookAccount = 404,
    ListFriendFacebookIds = 405,
    ListPlayerAccountsByFacebookIds = 406,
    GetAllUserStoreItems = 407,
    AddQuestItemNotify = 408,
    SetPlayerFindable = 409,
    GetPlayerChatReceivedHistory = 410,
    GetAllStoreThemes = 411,
    SellItem = 412,
    GetAllBuildings = 413,
    GetAllItemcategories = 414,
    PlantPlayerItem = 415,
    GetFeaturedItems = 416,
    GetNpcRelationshipLevels = 417,
    SaveGameState = 418,
    ListSavedGames = 419,
    LoadGame = 420,
    SendGame = 421,
    AcceptGame = 422,
    RejectGame = 423,
    CmsGetGameState = 427,
    GetAllFeaturedItems = 428,
    ManageNpcFriendRequest = 429,
    PlaceVillageItem = 430,
    GetCmsVillageRoles = 431,
    SendEmailMessage = 432,
    ListInboxMessages = 433,
    GetInboxMessage = 434,
    MarkInboxMessage = 435,
    GetRuleCount = 436,
    ListSentMessages = 437,
    GetVillageRoles = 438,
    AssignVillageRole = 439,
    GetCmsVillageTemplatesLocked = 440,
    ConfirmFindable = 441,
    CreatePrivateVillage = 442,
    GetRuleInfoList = 443,
    GetRuleInstanceInfoList = 444,
    GetOtherPlayerDetailsList = 445,
    AttachItems = 446,
    DetachItems = 447,
    AddAvatarCstool = 448,
    KickOut = 449,
    HarvestPlayerItem = 450,
    GetNpcGifts = 451,
    RemoveVillageItem = 452,
    ListVillageItems = 453,
    GetCmsVillageTemplatesUnlocked = 454,
    ListFindablePlayers = 455,
    GetAllCraftableItems = 456,
    ListCmsMessages = 457,
    GetCmsMessage = 458,
    GetFriendshipRequestCounts = 459,
    GetNpcInteractions = 460,
    GetCraftableTypes = 461,
    VillageInviteStatusNotify = 462,
    FindNpcsByName = 463,
    ChangePassword = 464,
    ForgotPassword = 465,
    SetPassword = 466,
    AcceptEventNotify = 467,
    ListQuestSpawnIds = 468,
    GetRuleInstanceInfoForUi = 469,
    GetRuleInfoListForUi = 470,
    SendTestMessage = 471,
    SaveAvatarImage = 472,
    GetAvatarImages = 473,
    GetObjectsLockInfo = 474,
    GetRuleTemplateIdForUi = 475,
    CraftItemsByCraftableItemIds = 476,
    GetAllGames = 477,
    UpdateGenderAndLocation = 478,
    GetGeographicalLocations = 479,
    GetCannedMessageCategories = 480,
    PreFilterNameCheckAvailability = 481,
    CraftBracelet = 482,
    GetNpcItems = 483,
    ItemInteraction = 484,
    CreateQuestGame = 485,
    EnterMaze = 486,
    RemoveAndDetachMazeItem = 487,
    AttachAndPlaceItem = 488,
    StartCraftingBracelet = 489,
    GetQuestFromParent = 490,
    CreateQuest = 491,
    GetPlayerAttachmentItemsBySenderId = 492,
    SetFriendOrder = 493,
    GetRuleInstanceData = 494,
    GetMembershipSubscriptionById = 495,
    ClaimMembershipSubscription = 496,
    GetUserSubscriptionInfo = 497,
    ProcessCrispMessage = 498,
    ListSendingPlayerAttachmentItems = 499,
    OidToDbid = 500,
    DbidToOid = 501,
    FilterBadWord = 502,
    DockItem = 503,
    GetMessagesCount = 504,
    GetViews = 505,
    DiscoverOnlineUser = 506,
    GetUnlockInfoPeer = 507,
    DeleteZingFromPlayer = 508,
    SendNotificationCsTool = 509,
    FriendStatusNotify = 510,
    GetRuleCountList = 511,
    SellPlayerItems = 512,
    SendMessageCsTool = 513,
    GiftStatusNotify = 514,
    LevelStatusNotify = 515,
    PlayerNotificationNotify = 516,
    GetSystemNotifications = 517,
    GetSystemMessages = 518,
    AttachSingleItem = 519,
    ManageCrispNotify = 520,
    AddPlayerAwardNotify = 521,
    GetStatefulInstance = 522,
    MarkAnnouncementRead = 523,
    ReorderFriendsOrdinal = 524,
    GetEstoreTransInfoForCstool = 525,
    GetEstorePointsForCstool = 526,
    GetPlayerExternalSiteMap = 527,
    SetPlayerExternalSiteMap = 528,
    UpdateTokenPlayerExternalSiteMap = 529,
    GetAuthenticationPlayerExternalSiteMap = 530,
    SendEmailAttachment = 531,
    SetBraceletsArmOrder = 532,
    SendNotificationTemplate = 533,
    CheckOnlineStatusTemplate = 534,
    GetOtherPlayerDetailsTemplate = 535,
    CrossUserServerPlayerTemplate = 536,
    CrossUserServerPlayerTemplateNotify = 537,
    GetPlayerInfoOnOtherShardTemplate = 538,
    MultiDarTemplate = 539,
    MultiDarComplexTemplate = 540,
    GetRandomNames = 541,
    SelectPlayerName = 542,
    ValidateName = 543,
    QuestEventInProgress = 544,
    GetQuestAllFromParent = 546,
    GetPlayerVotedList = 547,
    CreatePlayerVoted = 548,
    VoteOnPlayerVoted = 549,
    GetPlayerVoted = 550,
    GetComposedItem = 551,
    SaveComposedItem = 552,
    CompletePlayerVoted = 553,
    WithdrawInstanceForVoted = 554,
    GetLeaderBoardInfoForPlayerVoted = 555,
    GetFriendsPlayerVoteds = 556,
    ManageSynchronizedObjects = 557,
    PurchaseUnlock = 558,
    ValidateAndRedeemMobileProductPurchase = 559,
    GetPlayerVotedData = 560,
    UpdateAccountReferral = 561,
    RemovePlayerExternalSiteMapping = 562,
    GetInvitedPlayerQuest = 563,
    AcceptQuestTransaction = 564,
    FinalizeQuestTransaction = 565,
    GetClientVersionInfo = 566,
    PurchaseWalletItem = 567,
    GetPlayerOnlineStatus = 568,
    GetItemById = 569,
    GetPublicAssetsByOids = 570,
    GetPublicItemsByOids = 571,
    GetPublicItemCategories = 572,
    GetParentBuildingId = 573,
    CreateRecipe = 574,
    GetNpcsByChildHierarchies = 575,
    RecycleRecipe = 576,
    EnhanceRecipe = 577,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(i64)]
pub enum UserMessage2 {
    MfCrisp = 10,
    MfSynchronousApi = 11,
    MfAgent = 12,
}

#[derive(PartialEq, Eq, Hash, Debug, TryFromPrimitive)]
#[repr(i64)]
pub enum SyncMessage {
    AddObject = 1,
    MoveObject = 2,
    RemoveObject = 3,
    ChangeObject = 4,
    VillageHandoffQuery = 5,
    ServerChangeObject = 6,
    MovePlayer = 7,
    RemovePlayer = 8,
    Chat = 9,
    StartEvent = 10,
    StopEvent = 11,
    Emote = 12,
    BindUserNotify = 13,
    PauseNpc = 14,
    ResumeNpc = 15,
    UpdateNpcScript = 16,
    Notification = 17,
    ClientTest = 18,
    AddItem = 19,
    Evict = 20,
    EnterLoc = 21,
    ExitLoc = 22,
    Echo = 23,
    GetVillage = 24,
    Refresh = 25,
    Bind = 26,
    BindQuery = 27,
    BindVillageNotify = 28,
    VillageHandoff = 29,
    FindServer = 30,
    UpdateFilter = 31,
    SendNotify = 32,
    Login = 33,
    UpdateUserInfo = 34,
    UpdateUserFriends = 35,
    UpdateUserGroups = 36,
    ManageUserFriends = 37,
    ManageUserGroups = 38,
    ListServers = 39,
    ListVillages = 40,
    ListUsers = 41,
    MoveVillage = 42,
    StartNpcs = 43,
    StopNpcs = 44,
    UserSessionHandoff = 45,
    EmoteSvc = 46,
    Action = 47,
    Logout = 48,
    CloseZone = 49,
    TerminateUserSession = 50,
    GetPlayerCount = 51,
    UpdateLocation = 52,
    HeartbeatNotify = 54,
    Relogin = 55,
}

#[derive(Debug)]
pub enum AppCode {
    Ilg = -1,
    Ok = 0,
    Continue = 1,
    Err = 10,
    Type = 11,
    Interlock = 12,
    NoInterlock = 13,
    Input = 14,
    DupRequest = 15,
    SvcVer = 16,
    MultErrs = 17,
    NotFound = 18,
    NotImplemented = 19,
    Memory = 20,
    NotReady = 21,
    State = 22,
    Perm = 40,
    Xperm = 41,
    Auth = 42,
    Session = 43,
    NoSpace = 50,
    FilePath = 51,
    FileName = 52,
    File = 53,
    FileIo = 54,
    FileMode = 55,
    FileDamaged = 56,
    FileAccess = 57,
    FileNf = 58,
    FileRename = 59,
    FileMove = 60,
    FileCopy = 61,
    DB = 70,
    DupKey = 71,
    DbrcUnknown = 72,
    InvalidUser = 100,
    InvalidName = 101,
    InvalidRelationship = 102,
    InvalidPrimaryKey = 103,
    PinMismatch = 104,
    SiteCreationFailed = 105,
    InvalidItem = 106,
    NotSellableItem = 107,
    InvalidPlacement = 109,
    InsufficientFund = 110,
    InvalidInventorySearch = 111,
    InvalidFeatureCode = 112,
    InventoryItemNotExist = 113,
    NotLogIn = 114,
    DuplicateNickname = 115,
    PlayerCreationFailed = 116,
    PlayerDeletionFailed = 117,
    InvalidAuthentication = 118,
    BlankCredentials = 119,
    InvalidSiteInfo = 120,
    InvalidInventoryOrdinal = 121,
    CodeStatusUpdateFailed = 122,
    SecretCodeVerificationFailed = 123,
    InvalidObjectEstoreSku = 124,
    CreatePlayerAccountFailed = 125,
    InvalidPin = 126,
    InvalidAsset = 127,
    BackpackIsFull = 128,
    NoActiveVillager = 129,
    ItemNotInContainerOrBackpack = 130,
    ItemsInSameContainerOrBackpack = 131,
    InventoryTypeIsNull = 132,
    InventoryTypesNotSame = 133,
    SwappedItemsAreSame = 134,
    ItemNotOwnedBySessionPlayer = 135,
    InvalidSiteContent = 136,
    StarterTownIdNotConfigured = 137,
    PresentableSlotsUsedUp = 138,
    InvalidGameId = 139,
    InvalidToken = 140,
    TooManyPoints = 141,
    NoCheckpoint = 142,
    InavlidAction = 143,
    NameCannotBeEmpty = 160,
    NotAcceptPresentableItem = 161,
    PlayerHasNoHomeVillage = 162,
    OnlyOneSearchCriterionAllowed = 163,
    OneSearchCriterionMustBeProvided = 164,
    InvalidItemRelationName = 165,
    UnpaidAccount = 166,
    AlreadyQueued = 167,
    InvalidCooldown = 168,
    InvalidAuth = 169,
    InsufficientPermission = 170,
    AlreadyHaveATourist = 171,
    InvalidItemOrBuildingPlacement = 172,
    NotEnoughSpace = 173,
    CraftingFail = 174,
    ChildAlreadyPlaced = 175,
    SlotAlreadyUsed = 176,
    SlotNotOnParent = 177,
    IncompatibleSlot = 178,
    ParentNotCraftable = 179,
    InvalidOutfitNo = 180,
    InsufficientFunds = 181,
    InappropriateLanguage = 182,
    InvalidVillageName = 183,
    PendingVillageExists = 184,
    CreateVillageFailed = 185,
    PlayerDoesNotOwnInventory = 200,
    NotEmoteInventory = 201,
    InvalidEmote = 202,
    InvalidPlayerEmoteId = 203,
    InvalidCode = 210,
    ScsBlocked = 225,
    InternalError = 500,
    TestErrorCode = 9999,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(i64)]
pub enum ServiceClass {
    UserServer = 18,
    SyncServer = 19,
    Location = 20,
    Client = -1,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(i64)]
pub enum ResultCode {
    Incomplete = -1,
    OK = 0,
    App = 5,
    AppDb = 6,
    Err = 10,
    Queue = 11,
    DB = 20,
    DbQueue = 21,
    DbNoRetry = 22,
    NoMem = 30,
    Comm = 40,
    ConnFailed = 41,
    Disconnect = 42,
    Shutdown = 43,
    IO = 44,
    Timeout = 45,
    Busy = 46,
    CommInit = 47,
    Cancel = 48,
    WouldBlock = 49,
    ProtocolVer = 50,
    Serialize = 51,
    PendingIo = 52,
    Async = 53,
    Conn = 54,
    ChnlClosed = 55,
    ConnExhst = 56,
    NoDest = 57,
    Chrono = 58,
    NotReady = 59,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MessageType {
    User(UserMessage),
    Client(ClientMessage),
    Sync(SyncMessage),
}

#[derive(Debug)]
pub struct Message {
    pub flags: u32,
    pub message_type: MessageType,
    pub request_id: u32,
}

pub fn decode_message(buffer: &[u8]) -> Option<Message> {
    let (_, buffer) = get_message_data(buffer).unwrap();
    let buffer = (buffer, 0);
    let (buffer, (gsf_request_null, message_header_null)) = get_start_bits(buffer).unwrap();

    let (buffer, header) = cond(
        !message_header_null,
        tuple((get_number::<4>, get_message_type, get_number::<4>)),
    )(buffer)
    .unwrap();

    if let Some((flags, message_type, request_id)) = header {
        return Some(Message {
            flags: flags as u32,
            message_type,
            request_id: request_id as u32,
        });
    }

    None
}

pub fn get_start_bits(buffer: (&[u8], usize)) -> IResult<(&[u8], usize), (bool, bool)> {
    (bits::complete::bool, bits::complete::bool).parse(buffer)
}

pub fn get_message_data(buffer: &[u8]) -> IResult<&[u8], &[u8]> {
    let (buffer, length) = bytes::complete::take(1usize)(buffer)?;
    bytes::complete::take(length[0])(buffer)
}

pub fn get_message_type(buffer: (&[u8], usize)) -> IResult<(&[u8], usize), MessageType> {
    let (buffer, (service_class, message_type)) =
        (get_number::<4>, get_number::<4>).parse(buffer)?;

    let message_type = match ServiceClass::try_from(service_class).unwrap() {
        ServiceClass::UserServer => MessageType::User(UserMessage::try_from(message_type).unwrap()),
        ServiceClass::SyncServer => MessageType::Sync(SyncMessage::try_from(message_type).unwrap()),
        ServiceClass::Location => todo!(),
        ServiceClass::Client => MessageType::Client(ClientMessage::try_from(message_type).unwrap()),
    };

    Ok((buffer, message_type))
}

pub fn get_number<const BYTES: usize>(buffer: (&[u8], usize)) -> IResult<(&[u8], usize), i64> {
    let (buffer, result) = bits::complete::bool(buffer)?;

    let (buffer, bit_size) = if result {
        get_size::<BYTES>(buffer)?
    } else {
        (buffer, BYTES * 8)
    };

    let bit_array = buffer
        .0
        .view_bits::<Msb0>()
        .get(buffer.1..buffer.1 + bit_size)
        .unwrap();

    // HACK: Nom doesn't allow taking more than 8 bits at a time. sooo.........
    let (buffer, final_value) = match bit_size {
        4 | 8 => {
            let num = bit_array.load_be::<i64>();
            let (buffer, _): (_, u8) = bits::complete::take(bit_size)(buffer)?;
            (buffer, num)
        }
        16 => {
            let num = bit_array.load_be::<i64>();
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            (buffer, num)
        }
        32 => {
            let num = bit_array.load_be::<i64>();
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            (buffer, num)
        }
        64 => {
            let num = bit_array.load_be::<i64>();
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            let (buffer, _): (_, u8) = bits::complete::take(8usize)(buffer)?;
            (buffer, num)
        }
        _ => {
            panic!()
        }
    };

    Ok((buffer, final_value))
}

pub fn get_size<const MAX_BYTES: usize>(buffer: (&[u8], usize)) -> IResult<(&[u8], usize), usize> {
    let mut byte_size = 0;

    for bit in buffer.0.view_bits::<Msb0>().iter().skip(buffer.1) {
        if !bit {
            break;
        }
        byte_size += 1;
    }

    assert!(byte_size <= MAX_BYTES);

    // Skip ahead in nom
    let (buffer, _): (_, u64) = bits::complete::take(byte_size + 1)(buffer)?;

    Ok((buffer, if byte_size != 0 { byte_size * 8 } else { 4 }))
}
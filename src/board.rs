use crate::kilobot ::*;
use std::{fmt, mem};

pub const NORTH: u16 = 0;
pub const EAST: u16 = 90;
pub const SOUTH: u16 = 180;
pub const WEST: u16 = 270;

pub enum LocationError {
    AlreadyOccupied,
    NotOccupied,
    OutOfBounds,
}

pub struct Board
{
    width: u8,
    height: u8,
    bots: Vec<Option<BotLocation>>,         //2D array packed into a Vector
}

impl Board
{
    /// Add new bot to the board
    /// # Arguments
    /// 'bot' - Kilobot to add to the board
    /// 'x' - X coordinate to place the bot
    /// 'y' - Y coordinate to place the bot
    /// 'facing' - Direction the bot is initially facing, in degrees clockwise from north
    /// # Returns
    /// None - Insert successful
    /// LocationError if out of bounds or coordinates already occupied
    pub fn add_bot_location(&mut self, bot: Kilobot, x: u8, y: u8, facing: u16) -> Option<LocationError>
    {
        if x < self.width && y < self.height
        {
            let desired_index: usize;
            match self.get_index_from_coord(x, y)
            {
                Ok(index) => desired_index = index,
                Err(e) => return Some(e),
            }

            let mut desired_position = self.bots.get_mut(desired_index).unwrap().as_ref();

            match desired_position {
                Some(_) => Some(LocationError::AlreadyOccupied),
                None => {
                    mem::swap(&mut self.bots[desired_index], &mut Some(BotLocation { bot, facing }));
                    None
                }
            }
        }
        else { Some(LocationError::OutOfBounds) }
    }

    /// Removes the BotLocation at the specified coordinates if a bot is present there and replaces it with None
    /// Finds the index of the coordinate pair and calls remove_bot_location_at_index
    /// # Arguments
    /// * 'x' - X-coordinate of BotLocation
    /// * 'y' - Y-Coordinate of BotLocation
    /// # Returns
    /// Ok - Box<BotLocation> Pointer to removed BotLocation
    /// Err(LocationError) if coordinates are out of bounds or there is no bot in the coordinate
    pub fn remove_bot_location_at_coord(&mut self, x: u8, y: u8) -> Result<Box<BotLocation>,LocationError>
    {
            self.remove_bot_location_at_index(self.get_index_from_coord(x, y)?)
    }

    /// Removes the BotLocation at the specified index if a bot is present there and replaces it with None
    /// # Arguments
    /// * 'index' - Index of BotLocation to remove
    /// # Returns
    /// Ok - Box<BotLocation> Pointer to removed BotLocation
    /// Err(LocationError) if index is out of bounds or there is no bot in the coordinate
    pub fn remove_bot_location_at_index(&mut self, index: usize) -> Result<Box<BotLocation>,LocationError>
    {
        if index >= 0 && index < self.bots.len()
        {
            match self.bots.get(index)
            {
                Some(b) => {
                    let bot = mem::replace(&mut self.bots[index], None);
                    Ok(Box::new(bot.unwrap()))
                    // bot = Box::new(self.bots);
                    // Ok(Box::new(*bot))
                },
                None => Err(LocationError::NotOccupied),
            }

        }
        else { Err(LocationError::OutOfBounds) }
    }

    /// Returns an immutable reference to the bot at given coordinates, or LocationError if none
    /// Finds the index of the coordinates then calls get_bot_at_index(index)
    /// # Arguments
    /// * 'x' - X coordinate to check
    /// * 'y' - Y coordinate to check
    /// # Returns
    /// * Ok - Reference to Box<Kilobot>
    /// * Err - LocationError if no bot is found, or out of bounds
    pub fn get_bot_at_coord(&self, x: u8, y: u8) -> Result<&Kilobot, LocationError>
    {
        self.get_bot_at_index(self.get_index_from_coord(x, y)?)
    }

    /// Returns an immutable reference to the bot at given coordinates, or LocationError if none
    /// # Arguments
    /// * 'index' - Index of location in board array
    /// # Returns
    /// * Ok - Reference to a Kilobot
    /// * Err - LocationError if no bot is found, or out of bounds
    pub fn get_bot_at_index(&self, index: usize) -> Result<&Kilobot, LocationError>
    {
        if index < self.bots.len()
        {
            let this_location = self.bots.get(index).unwrap();
            match this_location
            {
                Some(_) => Ok(this_location.as_ref().unwrap().bot()),
                None => Err(LocationError::NotOccupied),
            }
        }
        else { Err(LocationError::OutOfBounds) }
    }

    /// Get the array index from an x and y coordinate
    /// # Arguments
    /// * 'x' - X coordinate
    /// * 'y' - Y coordinate
    /// # Returns
    /// Ok - usize index of desired x & y coordinate
    /// Err - LocationError if coordinates are out of bounds
    pub fn get_index_from_coord(&self, x: u8, y: u8) -> Result<usize, LocationError>
    {
        if x < self.width && y < self.height
        {
            Ok((x + (y * self.width)) as usize)
        }
        else { Err(LocationError::OutOfBounds) }

    }

    /// Print left to right, top to bottom
    pub fn print_board(&self)
    {
        for j in 0..self.height
        {
            for i in 0..self.width
            {
                let this_space = self.bots.get(match self.get_index_from_coord(i, j) {
                    Ok(x) => x,
                    Err(_) => unimplemented!(),
                }).unwrap();
                match this_space
                {
                    Some(loc) => print!("  {}   ", loc.bot.get_uid()),
                    None => print!("({},{}) ", i, j),
                }
            }
            print!("\n\n");
        }
    }
}

pub struct BotLocation
{
    bot: Kilobot,
    facing: u16,            //Represents the current angle of the bot, where 0 is north
}

impl BotLocation
{
    /// Return an immutable reference to the bot in the location
    /// Allows accessing the bot functions, but cannot change bot values
    /// # Returns
    /// * Immutable reference to the bot in the Location
    pub fn bot(&self) -> &Kilobot
    {
        &self.bot
    }

    /// Return a mutable reference to the bot in the location
    /// Allows accessing the bot functions and changing bot values
    /// # Returns
    /// * Mutable reference to the bot in the Location
    pub fn bot_mut(&mut self) -> &mut Kilobot
    {
        &mut self.bot
    }

    /// Return the facing of the bot in the location
    /// # Returns
    /// * The rotation of the bot in degrees clockwise away from north
    pub fn get_facing(&self) -> u16
    {
        self.facing
    }

    /// Sets the facing of the bot in the location
    /// # Arguments
    /// * 'new_facing' - The new facing of the bot, in degrees clockwise from north
    pub fn set_facing(&mut self, new_facing: u16)
    {
        self.facing = new_facing
    }
}

/// Create a new instance of Board and fill it with empty Locations
/// # Arguments
/// * 'width' - How wide the board should be
/// * 'height' - How tall the board should be
///
/// For example, new_board(4,3) would create a 4x3 board that looks like this
///             * * * *
///             * * * *
///             * * * *
///         where '*' represents "None"
pub fn new_board(width: u8, height: u8) -> Board
{
    let mut board = Board {width, height, bots: Vec::with_capacity((width * height).into())};
    for _i in 0..width * height
    {
        board.bots.push(None);
    }
    return board;
}

impl fmt::Display for Board
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut num_bots: u16 = 0;
        for index in 0..self.bots.len()
        {
            if self.bots.get(index).unwrap().is_some()
            {
                num_bots += 1;
            }
        }
        write!(f, "(width:{}, height:{}, number of bots:{})"
               , self.width
               , self.height
               , num_bots)
    }
}

impl fmt::Display for BotLocation
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "[Bot: {}, Facing: {}]"
                , self.bot
                , self.facing)
    }
}
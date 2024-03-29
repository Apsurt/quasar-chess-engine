"""
This module contains the Board class, 
which is responsible for managing the state of the game board.
"""

import math
from typing import Tuple, Generator, List
from quasar.logger import logger, silence, unsilence
from quasar.chess.moves import Move
from quasar.chess.errors import NonePieceError, InvalidMoveError, InvalidPlayerError
from quasar.chess.point import Point
from quasar.chess.pieces import Piece, PieceFactory, PieceColor, PieceName
from quasar.chess.utils import fen_to_piece_name

class Board:
    """
    The Board class is responsible for managing the state of the game board.
    """
    def __init__(self) -> None:
        """
        The constructor for the Board class.
        """
        self.pieces = []
        self.captured_pieces = []
        self.moves = []

        self.current_player = PieceColor.WHITE

        self.factory = PieceFactory()
        self.none_piece = self.factory.create_piece(PieceName.NONE, Point(0, 0), PieceColor.NONE)

        self.validator = Validator()
        self.last_concentration = Point(0, 0)

    def create_piece(self, name: PieceName, position: Point, color: PieceColor) -> Piece:
        """
        Create a piece and add it to the board.

        :param name: Name of the piece to create.
        :type name: PieceName
        :param position: The position of the piece.
        :type position: Point
        :param color: The color of the piece.
        :type color: PieceColor
        :return: The created piece.
        :rtype: Piece
        """
        piece = self.factory.create_piece(name, position, color)
        self.add_piece(piece)
        return piece

    def load_fen(self, fen: str) -> None:
        """
        Load a FEN string into the board.

        :param fen: The FEN string to load.
        :type fen: str
        """
        placement, turn, castling, en_passant, halfmove, fullmove = fen.split(" ")
        del turn, castling, en_passant, halfmove, fullmove
        placement = placement.split("/")
        for y, row in enumerate(placement):
            y = 8 - y
            for x, char in enumerate(row):
                if char.isdigit():
                    x += int(char)
                else:
                    color = PieceColor.WHITE if char.isupper() else PieceColor.BLACK
                    piece_name = PieceName[fen_to_piece_name(char)]
                    self.create_piece(piece_name, Point(x+1, y), color)

    def change_player(self) -> None:
        """
        Change the current player.
        """
        if self.current_player == PieceColor.WHITE:
            self.current_player = PieceColor.BLACK
        elif self.current_player == PieceColor.BLACK:
            self.current_player = PieceColor.WHITE
        else:
            raise InvalidPlayerError(
                f"Current player has to be either WHITE or BLACK. Got {self.current_player.name}")

    def get_pieces(self) -> list:
        """
        Get the pieces on the board.

        :return: The pieces on the board.
        :rtype: list
        """
        return self.pieces

    def add_piece(self, piece: Piece) -> None:
        """
        Add a piece to the board.

        :param piece: The piece to add to the board.
        :type piece: Piece
        """
        self.pieces.append(piece)

    def remove_piece(self, piece: Piece) -> None:
        """
        Remove a piece from the board.

        :param piece: The piece to remove from the board.
        :type piece: Piece
        """
        self.pieces.remove(piece)

    def clear(self) -> None:
        """
        Clear the board.
        """
        self.clear_pieces()
        self.clear_moves()
        self.clear_captured_pieces()
        self.current_player = PieceColor.WHITE

        self.factory = PieceFactory()
        self.none_piece = self.factory.create_piece(PieceName.NONE, Point(0, 0), PieceColor.NONE)

        self.validator = Validator()
        self.last_concentration = Point(0, 0)

    def clear_pieces(self) -> None:
        """
        Clear the pieces from the board.
        """
        self.pieces = []

    def clear_moves(self) -> None:
        """
        Clear the moves from the board.
        """
        self.moves = []

    def clear_captured_pieces(self) -> None:
        """
        Clear the captured pieces from the board.
        """
        self.captured_pieces = []

    def get_concentration_position(self) -> Point:
        """
        Get the concentration position of the pieces on the board.
        Currently, it is not used.

        :return: The concentration position of the pieces on the board.
        :rtype: Point
        """
        new_concentration = self.last_concentration.copy()
        for piece in self.pieces:
            d = abs(piece.get_position() - self.last_concentration)
            d = Point(math.sqrt(d.x), math.sqrt(d.y))
            new_concentration += d
        new_concentration /= len(self.pieces)
        self.last_concentration = new_concentration
        return self.last_concentration

    def get_white_pieces(self) -> list:
        """
        Get the white pieces on the board.

        :return: The white pieces on the board.
        :rtype: list
        """
        return [piece for piece in self.pieces if piece.get_color() == PieceColor.WHITE]

    def get_black_pieces(self) -> list:
        """
        Get the black pieces on the board.

        :return: The black pieces on the board.
        :rtype: list
        """
        return [piece for piece in self.pieces if piece.get_color() == PieceColor.BLACK]

    def get_piece_at(self, position: Point) -> Piece:
        """
        Get the piece at a position on the board.

        :param position: The position to get the piece from.
        :type position: Point
        :return: The piece at the position.
        :rtype: Piece
        """
        for piece in self.pieces:
            if piece.get_position() == position:
                return piece
        return self.none_piece

    def find_pieces(self, name: PieceName, color: PieceColor) -> List[Piece]:
        """
        Find pieces on the board.

        :param name: Name of the piece to find.
        :type name: PieceName
        :param color: Color of the piece to find.
        :type color: PieceColor
        :return: The pieces found.
        :rtype: List[Piece]
        """
        pieces = []
        for piece in self.pieces:
            if piece.name == name and piece.color == color:
                pieces.append(piece)
        return pieces

    def get_possible_moves_generator(
        self, piece: Piece,
        bottom_left_bound: Point = Point(-999,-999),
        top_right_bound: Point = Point(999,999)
        ) -> Generator[Move, None, None]:
        """
        _summary_

        :param piece: _description_
        :type piece: Piece
        """
        silence()
        #if piece.color != self.current_player:
        #    raise InvalidPlayerError(
        #        f"Current player is {self.current_player.name}, but piece is {piece.color.name}")
        offset_generator = piece.get_offset_generator(bottom_left_bound, top_right_bound)
        misfire = 0
        while misfire < 100:
            try:
                offset = next(offset_generator)
            except StopIteration:
                break
            target = piece.get_position() + offset
            if bottom_left_bound.x <= target.x <= top_right_bound.x and \
                bottom_left_bound.y <= target.y <= top_right_bound.y:
                move = Move(piece.get_color(), piece.get_position(), target)
                move, is_legal = self.validator(move, self, True)
                if is_legal:
                    yield move
            else:
                misfire += 1
        if piece.is_king():
            castling_moves = [
            Move(piece.get_color(), piece.get_position(), piece.get_position() + Point(2,0)),
            Move(piece.get_color(), piece.get_position(), piece.get_position() + Point(-2,0))]
            for move in castling_moves:
                move, is_legal = self.validator(move, self, True)
                if is_legal:
                    yield move
        unsilence()

    def is_possible_move(self, move_to_check: Move) -> bool:
        """
        Check if a move is possible.

        :param move_to_check: The move to check.
        :type move_to_check: Move
        :return: True if the move is possible, False otherwise.
        :rtype: bool
        """
        move, is_legal = self.validator(move_to_check, self)
        piece = move.moved
        target = move.target
        if is_legal:
            generator = self.get_possible_moves_generator(piece)
            while True:
                try:
                    move = next(generator)
                    if move.target == target:
                        return True
                except StopIteration:
                    return False
        return False

    def capture(self, piece: Piece) -> None:
        """
        Capture a piece on the board.

        :param piece: The piece to capture.
        :type piece: Piece
        """
        self.captured_pieces.append(piece)
        self.pieces.remove(piece)

    def make_move(self, move: Move, check_if_legal: bool = True) -> None:
        """
        Makes a move on the board.

        :param validated_move: The move to make. Has to be legal.
        :type validated_move: Move
        :param check_if_legal: If the move should be checked if it is legal.
        :type check_if_legal: bool
        :raises InvalidMoveError: If the move is not legal.
        """

        if check_if_legal:
            legal_move, is_legal = self.validator(move, self)
        else:
            legal_move = move
            is_legal = True

        if not is_legal:
            raise InvalidMoveError()

        self.change_player()

        self.moves.append(legal_move)
        legal_move.moved.set_position(legal_move.target)
        legal_move.moved.moved = True
        try:
            legal_move.moved.update_offsets()
        except AttributeError:
            pass
        if legal_move.captured != self.none_piece:
            self.capture(legal_move.captured)
        if legal_move.flags.castling:
            offset = legal_move.target - legal_move.source
            if offset.x > 0:
                rook = self.get_piece_at(legal_move.source + Point(3,0))
                rook.set_position(legal_move.source + Point(1,0))
                rook.moved = True
            else:
                rook = self.get_piece_at(legal_move.source + Point(-4,0))
                rook.set_position(legal_move.source + Point(-1,0))
                rook.moved = True

    def undo_move(self) -> None:
        """
        Undo the last move made on the board.
        """
        move = self.moves.pop()
        move.moved.set_position(move.source)
        if move.captured != self.none_piece:
            self.pieces.append(move.captured)
        move.moved.moved = False
        self.change_player()
        try:
            move.moved.update_offsets()
        except AttributeError:
            pass

    def is_in_check(self, color: PieceColor) -> bool:
        """
        Check if a player is in check.

        :param color: The color of the player to check.
        :type color: PieceColor
        :return: True if the player is in check, False otherwise.
        :rtype: bool
        """
        king = self.find_pieces(PieceName.KING, color)[0]
        for piece in self.pieces:
            if piece.color != color:
                generator = self.get_possible_moves_generator(piece, king.position, king.position)
                for move in generator:
                    if move.target == king.position:
                        return True
        return False

    def is_in_checkmate(self, color: PieceColor) -> bool:
        return False

    def print(self) -> None:
        """
        Print the board to the console.
        """
        for y in range(8, 0, -1):
            for x in range(1, 9, 1):
                piece = self.get_piece_at(Point(x, y))
                print(piece.get_fen_char(), end=" ")
            print()

class Validator:
    """
    This class is responsible for validating moves.
    """
    def __call__(self,
                 move_to_validate: Move,
                 board_state: Board,
                 in_generator: bool = False
                ) -> Tuple[Move, bool]:
        """
        This method performs all the logic to categorize move as legal or illegal.

        :param move: Move to be validated.
        :type move: Move
        :param board: Board on which the move is to be played.
        :type board: Board
        :param in_generator: If the method is called from a generator.
        :type in_generator: bool
        :return: Validated move.
        :rtype: Move
        """
        move_to_validate.moved = board_state.get_piece_at(move_to_validate.source)
        move_to_validate.captured = board_state.get_piece_at(move_to_validate.target)

        if not self.is_move_legal(move_to_validate, board_state, in_generator):
            move_to_validate.legal = False
        else:
            move_to_validate.legal = True

        return move_to_validate, move_to_validate.legal

    def is_move_legal(self, move: Move, board: Board, in_generator: bool = False) -> bool:
        """
        This method checks if a move is legal.

        :param move: move to be checked.
        :type move: Move
        :param board: board on which the move is to be played.
        :type board: Board
        :param in_generator: If the method is called from a generator.
        :type in_generator: bool
        :raises NonePieceError: If there is no piece at the source of the move.
        :return: True if the move is legal, False otherwise.
        :rtype: bool
        """
        piece = move.moved
        offset = move.target - move.source

        if piece is board.none_piece:
            logger.error("No piece at %s", move.source)
            raise NonePieceError(f"No piece at {move.source}")

        if piece.name == PieceName.PAWN:
            if move.captured.name == PieceName.NONE:
                if abs(offset) == Point(1,1):
                    logger.warning("%s | Pawn can't move diagonally without capturing", str(move))
                    return False
            else:
                if abs(offset) != Point(1,1):
                    logger.warning("%s | Pawn can't move forward without capturing", str(move))
                    return False

        if piece.color == move.captured.color:
            logger.warning("%s | Can't capture own piece", str(move))
            return False

        if piece.is_king():
            if offset == Point(2,0) and \
            not piece.moved and \
            not board.get_piece_at(move.source + Point(3,0)).moved and \
            board.get_piece_at(move.source + Point(3,0)).name == PieceName.ROOK:
                move.flags.castling = True
            if offset == Point(-2,0) and \
            not piece.moved and \
            not board.get_piece_at(move.source + Point(-4,0)).moved and \
            board.get_piece_at(move.source + Point(-4,0)).name == PieceName.ROOK:
                move.flags.castling = True
            if move.flags.castling:
                logger.info("%s | Castling", str(move))

        if piece.is_sliding() or (piece.is_king() and move.flags.castling):
            source = move.source.copy()
            target = move.target.copy()
            direction = target - source
            direction.x = 1 if direction.x > 0 else -1 if direction.x < 0 else 0
            direction.y = 1 if direction.y > 0 else -1 if direction.y < 0 else 0
            source += direction
            while source != target:
                if board.get_piece_at(source).name != PieceName.NONE:
                    logger.warning("%s | Path is blocked by %s",
                                   str(move),
                                   board.get_piece_at(source).name.name)
                    return False
                source += direction

        if piece.is_king():
            if offset == Point(-2,0):
                if board.get_piece_at(move.source + Point(-3,0)).name != PieceName.NONE:
                    logger.warning("%s | Can't castle through pieces", str(move))
                    return False

        if move.source == move.target:
            log_msg = f"{str(move)} | Source and target are the same"
            logger.warning(log_msg)
            return False

        if move.source != piece.position:
            log_msg = f"{str(move)} | Source and piece position are different"
            logger.warning(log_msg)
            return False

        if not piece.sliding:
            if move.target not in [piece.position + offset for offset in piece.offsets]:
                if not move.flags.castling:
                    log_msg = f"{str(move)} | Move not in piece's offsets"
                    logger.warning(log_msg)
                    return False

        board.make_move(move, False)
        
        enemy_pieces = board.get_black_pieces() if piece.color == PieceColor.WHITE else board.get_white_pieces()
        
        for enemy_piece in enemy_pieces:
            offset_generator = enemy_piece.get_offset_generator(Point(1,1), Point(8,8))
            for offset in offset_generator:
                target_checked = board.get_piece_at(enemy_piece.position + offset)
                if not target_checked.is_king():
                    continue
                if enemy_piece.is_sliding():
                    source = enemy_piece.position.copy()
                    target = target_checked.position.copy()
                    direction = target - source
                    direction.x = 1 if direction.x > 0 else -1 if direction.x < 0 else 0
                    direction.y = 1 if direction.y > 0 else -1 if direction.y < 0 else 0
                    source += direction
                    while source != target:
                        if board.get_piece_at(source).name != PieceName.NONE:
                            break
                        source += direction
                    if source != target:
                        continue
                    if target_checked.is_king() and \
                        enemy_piece.color != target_checked.color:
                        logger.warning("%s | Can't move, sliding checks.", str(move))
                        board.undo_move()
                        return False
                if enemy_piece.is_pawn() and abs(offset) == Point(1,1):
                    if target_checked.is_king() and \
                        enemy_piece.color != target_checked.color:
                        logger.warning("%s | Can't move, pawn checks.", str(move))
                        board.undo_move()
                        return False
                elif enemy_piece.is_knight():
                    if target_checked.is_king() and \
                        enemy_piece.color != target_checked.color:
                        logger.warning("%s | Can't move, knight checks.", str(move))
                        board.undo_move()
                        return False
        
        board.undo_move()

        return True

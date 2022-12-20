#pragma once
#ifndef _TABLE_COLVIEW_COMMON_HPP_
#define _TABLE_COLVIEW_COMMON_HPP_

#include "tables.hpp"
#include "column_view.hpp"

ColView Table::operator[](const std::string &val) {
    uint16_t index = std::find(header.begin(), header.end(), val) - header.begin();
    return { header[index].c_str(), index, this };
}

#endif